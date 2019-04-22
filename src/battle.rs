use crate::ailments;
use crate::hooks::Hooks;
use crate::team::TeamMember;
use std::cell::RefCell;
use std::rc::Rc;
use vdex::Enum;
use vdex::moves;
use vdex::pokemon::OneOrTwo;
use vdex::Stat;
use vdex::Type;

#[derive(Clone, Debug)]
pub struct BenchPokemon {
    pub base: Rc<TeamMember>,
    pub status: ailments::BenchAilment,
    pub hp: u16,
}

impl BenchPokemon {
    pub fn new(base: &Rc<TeamMember>) -> Self {
        Self {
            base: base.clone(),
            status: Default::default(),
            hp: base.stat(Stat::HP),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BattlePokemon {
    pub index: usize,
    pub perm: Rc<RefCell<BenchPokemon>>,
    pub hooks: Hooks,
    pub overlay: TeamMember,
    pub types: OneOrTwo<Type>,
    pub status: ailments::BattlerAilments,
    pub stat_changes: [i8; moves::CHANGEABLE_STATS],
}

impl BattlePokemon {
    pub fn new(
        index: usize, perm: &Rc<RefCell<BenchPokemon>>, hooks: &Hooks
    ) -> Self {
        Self {
            index,
            perm: perm.clone(),
            hooks: Hooks::new_overlay(hooks),
            overlay: (*perm.borrow().base).clone(),
            types: perm.borrow().base.pokemon.types,
            status: Default::default(),
            stat_changes: [0; moves::CHANGEABLE_STATS],
        }
    }
}

impl BattlePokemon {
    pub fn direct_damage(&mut self, dmg: u16) {
        let mut perm = self.perm.borrow_mut();
        perm.hp = perm.hp.checked_sub(dmg).unwrap_or(0);
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
                match change {
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
                    _ => panic!("Warn: Stat change out of range"),
                }
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct DamageContext {
    pub user: Rc<RefCell<BattlePokemon>>,
    pub target: Rc<RefCell<BattlePokemon>>,
    pub slot: u8,
    pub mov: &'static moves::Move,
    pub typ: Type,
    pub power: u8,
    pub target_count: u8,
    pub class: moves::DamageClass,
    pub critical: bool,
}

impl DamageContext {
    pub fn calc_max_damage(&self) -> u16 {
        let user = self.user.borrow();
        let target = self.target.borrow();

        let level_factor = ((2 * user.overlay.level) / 5) + 2;
        let power = user.hooks.power_modifiers.fold(
            self.power as f64, |pow, func| pow * func.0(self)).trunc();

        let attack_stat = user.stat(match self.class {
            moves::DamageClass::Special => Stat::SpecialAttack,
            _ => Stat::Attack,
        }, self.critical);
        let attack = user.hooks.attack_modifiers.fold(
            attack_stat as f64, |atk, func| atk * func.0(self)).trunc();

        let defense_stat = target.stat(match self.class {
            moves::DamageClass::Special => Stat::SpecialDefense,
            _ => Stat::Defense,
        }, self.critical);
        let defense = target.hooks.defense_modifiers.fold(
            defense_stat as f64, |def, func| def * func.0(self)).trunc();

        let base_modi = if self.critical { 2.0 } else { 1.0 }
            * if self.target_count > 1 { 0.75 } else { 1.0 }
            * user.stab(self.typ) * target.efficacy(self.typ);
        let user_modi = user.hooks.user_damage_modifiers.fold(
            base_modi, |modi, func| modi * func.0(self));
        let target_modi = target.hooks.target_damage_modifiers.fold(
            user_modi, |modi, func| modi * func.0(self));

        let max = (((((((level_factor as f64) * power) * attack)
            / defense).trunc() / 50.0).trunc() + 2.0) * target_modi).trunc();
        max.min(std::u16::MAX as f64) as u16
    }

    pub fn damage<R: rand::Rng>(&self, rng: &mut R) -> u16 {
        let max = self.calc_max_damage();
        let dmg = ((max * rng.gen_range(85, 101)) / 100).max(1);
        self.target.borrow_mut().direct_damage(dmg);
        dmg
    }
}
