use crate::ailments;
use crate::hooks::Hooks;
use crate::formats::AbsoluteTarget;
use crate::team::TeamMember;
use std::cell::RefCell;
use std::rc::Rc;
use vdex::Enum;
use vdex::moves;
use vdex::pokemon::OneOrTwo;
use vdex::Stat;
use vdex::Type;

pub type Benched = Rc<RefCell<BenchPokemon>>;
pub type Bench = Vec<Benched>;
pub type Current = Rc<RefCell<BattlePokemon>>;

#[derive(Clone, Debug)]
pub struct BenchPokemon {
    pub base: Rc<TeamMember>,
    pub status: ailments::BenchAilment,
    pub hp: u16,
    pub pp: [u8; 4],
}

impl BenchPokemon {
    pub fn new(base: &Rc<TeamMember>) -> Self {
        Self {
            base: base.clone(),
            status: Default::default(),
            hp: base.stat(Stat::HP),
            pp: [base.max_pp(0), base.max_pp(1), base.max_pp(2), base.max_pp(3)],
        }
    }
}

#[derive(Clone, Debug)]
pub struct BattlePokemon {
    pub position: AbsoluteTarget,
    pub index: usize,
    pub perm: Benched,
    pub hooks: Hooks,
    pub overlay: TeamMember,
    pub types: OneOrTwo<Type>,
    pub status: ailments::BattlerAilments,
    pub stat_changes: [i8; moves::CHANGEABLE_STATS],
    pub critical_rate: i8,
}

impl BattlePokemon {
    pub fn new(
        position: AbsoluteTarget, index: usize, perm: &Benched, hooks: &Hooks
    ) -> Self {
        Self {
            position,
            index,
            perm: perm.clone(),
            hooks: Hooks::new_overlay(hooks),
            overlay: (*perm.borrow().base).clone(),
            types: perm.borrow().base.pokemon.types,
            status: Default::default(),
            stat_changes: [0; moves::CHANGEABLE_STATS],
            critical_rate: 0,
        }
    }

    pub fn direct_damage(&mut self, amt: u16) -> u16 {
        let mut perm = self.perm.borrow_mut();
        let capped = amt.min(perm.hp);
        perm.hp -= capped;
        capped
    }

    pub fn direct_heal(&mut self, amt: u16) -> u16 {
        let mut perm = self.perm.borrow_mut();
        let capped = amt.min(self.overlay.stat(Stat::HP) - perm.hp);
        perm.hp += capped;
        capped
    }

    pub fn direct_percentage(&mut self, base: u16, percent: i8) -> u16 {
        let amt = (base.checked_mul(percent.abs() as u16)
            .unwrap_or(std::u16::MAX) / 100).max(1);
        if percent > 0 {
            self.direct_heal(amt)
        } else {
            self.direct_damage(amt)
        }
    }

    pub fn change_stats(&mut self, changes: [i8; moves::CHANGEABLE_STATS]) {
        for i in 0..moves::CHANGEABLE_STATS {
            let change = changes[i].max(-12).min(12);
            let stat = &mut self.stat_changes[i];
            *stat = (*stat + change).max(-6).min(6);
        }
    }

    pub fn efficacy(&self, typ: Type) -> f64 {
        let dex = vdex::pokedex();
        let eff = dex.efficacy[(typ, self.types.first())].modifier();
        if let Some(second) = self.types.second() {
            eff * dex.efficacy[(typ, second)].modifier()
        } else {
            eff
        }
    }

    pub fn stab(&self, typ: Type) -> f64 {
        if self.types.contains(typ) {
            if self.overlay.ability == vdex::Ability::Adaptability {
                2.0
            } else {
                1.5
            }
        } else {
            1.0
        }
    }

    pub fn stat(&self, stat: Stat, critical: bool) -> u16 {
        match stat {
            Stat::HP => self.perm.borrow().hp,
            Stat::Accuracy | Stat::Evasion => 1,
            _ => {
                let base = self.overlay.stat(stat);
                let mut change = self.stat_changes[stat.repr() as usize];
                if critical {
                    change = match stat {
                        Stat::Attack | Stat::SpecialAttack => change.max(0),
                        Stat::Defense | Stat::SpecialDefense => change.min(0),
                        _ => change,
                    };
                }
                match change.max(-6).min(6) {
                    -6 => base / 4,
                    -5 => (base * 2) / 7,
                    -4 => base / 3,
                    -3 => (base * 2) / 5,
                    -2 => base / 2,
                    -1 => (base * 2) / 3,
                    0 => base,
                    1 => (base * 3) / 2,
                    2 => base * 2,
                    3 => (base * 5) / 2,
                    4 => base * 3,
                    5 => (base * 7) / 2,
                    6 => base * 4,
                    _ => unreachable!(),
                }
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct DamageContext {
    pub user: Current,
    pub target: Current,
    pub slot: u8,
    pub mov: &'static moves::Move,
    pub typ: Type,
    pub power: u8,
    pub target_count: u8,
    pub class: moves::DamageClass,
    pub critical: bool,
}

impl DamageContext {
    pub fn accuracy(&self) -> f64 {
        let user = self.user.borrow();
        let target = self.target.borrow();
        if let Some(base_percent) = self.mov.accuracy {
            let base = base_percent as f64 / 100.0;
            let stat_change = (user.stat_changes[Stat::Accuracy.repr() as usize]
                - target.stat_changes[Stat::Evasion.repr() as usize]).max(-6).min(6);
            let stat_modi = base * match stat_change {
                -6 => 0.33,
                -5 => 0.36,
                -4 => 0.43,
                -3 => 0.50,
                -2 => 0.60,
                -1 => 0.75,
                0 => 1.00,
                1 => 1.33,
                2 => 1.66,
                3 => 2.00,
                4 => 2.50,
                5 => 2.66,
                6 => 3.00,
                _ => unreachable!(),
            };
            let user_modi = user.hooks.user_accuracy_modifiers.fold(
                stat_modi, |modi, func| modi * func.0(self));
            let target_modi = target.hooks.target_accuracy_modifiers.fold(
                user_modi, |modi, func| modi * func.0(self));
            target_modi
        } else {
            1.0
        }
    }

    pub fn calc_max_damage(&self) -> u16 {
        let user = self.user.borrow();
        let target = self.target.borrow();
        let critical = target.hooks.critical_cancels.fold(
            self.critical, |crit, cancel| crit && !cancel);

        let level_factor = ((2 * user.overlay.level) / 5) + 2;
        let power = user.hooks.power_modifiers.fold(
            self.power as f64, |pow, func| pow * func.0(self)).trunc();

        let attack_stat = user.stat(match self.class {
            moves::DamageClass::Special => Stat::SpecialAttack,
            _ => Stat::Attack,
        }, critical);
        let attack = user.hooks.attack_modifiers.fold(
            attack_stat as f64, |atk, func| atk * func.0(self)).trunc();

        let defense_stat = target.stat(match self.class {
            moves::DamageClass::Special => Stat::SpecialDefense,
            _ => Stat::Defense,
        }, critical);
        let defense = target.hooks.defense_modifiers.fold(
            defense_stat as f64, |def, func| def * func.0(self)).trunc();

        let efficacy = target.efficacy(self.typ);
        if efficacy == 0.0 {
            0
        } else {
            let base_modi = if critical { 2.0 } else { 1.0 }
                * if self.target_count > 1 { 0.75 } else { 1.0 }
                * user.stab(self.typ) * efficacy;
            let user_modi = user.hooks.user_damage_modifiers.fold(
                base_modi, |modi, func| modi * func.0(self));
            let target_modi = target.hooks.target_damage_modifiers.fold(
                user_modi, |modi, func| modi * func.0(self));

            let max = (((((((level_factor as f64) * power) * attack) / defense)
                .trunc() / 50.0).trunc() + 2.0) * target_modi).trunc();
            (max.min(std::u16::MAX as f64) as u16).max(1)
        }
    }

    pub fn damage<R: rand::Rng>(&self, rng: &mut R) -> u16 {
        let max = self.calc_max_damage();
        let dmg = ((max * rng.gen_range(85, 101)) / 100).max(1).min(max);
        if dmg > 0 {
            self.target.borrow_mut().direct_damage(dmg)
        } else {
            0
        }
    }
}
