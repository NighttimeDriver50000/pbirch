use crate::ailments;
use crate::hooks;
use crate::team;
use vdex::Enum;
use vdex::moves;
use vdex::pokemon::OneOrTwo;
use vdex::Stat;
use vdex::Type;

pub struct BenchPokemon {
    pub base: team::TeamMember,
    pub status: ailments::BenchAilments,
    pub hp: u16,
}

pub struct BattlePokemon<'bat> {
    pub index: usize,
    pub perm: &'bat mut BenchPokemon,
    pub overlay: team::TeamMember,
    pub types: OneOrTwo<Type>,
    pub status: ailments::BattlerAilments,
    pub stat_changes: [i8; moves::CHANGEABLE_STATS],
}

pub struct MoveContext<'bat> {
    pub mov: &'static moves::Move,
    pub cls: moves::DamageClass,
    pub typ: Type,
    pub targets: u8,
    pub critical: bool,
    pub user: &'bat BattlePokemon<'bat>,
    pub bench: &'bat Vec<BenchPokemon>,
    pub hooks: &'bat hooks::Hooks,
}

impl<'bat> BattlePokemon<'bat> {
    pub fn efficacy(&self, typ: Type) -> f64 {
        let dex = vdex::pokedex();
        let eff = dex.efficacy[(typ, self.types.first())].modifier();
        if let Some(second) = self.types.second() {
            eff * dex.efficacy[(typ, second)].modifier()
        } else {
            eff
        }
    }

    pub fn stat(&self, stat: Stat, critical: bool) -> u16 {
        match stat {
            Stat::HP => self.perm.hp,
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

    pub fn calc_max_damage(&self, context: &MoveContext) -> u16 {
        let level_factor = ((2 * context.user.overlay.level) / 5) + 2;
        let power = context.hooks.power_modifiers.values().fold(
            context.mov.power as f64, |pow, func| pow * func(context)).trunc();
        let attack_stat = context.user.stat(match context.cls {
            moves::DamageClass::Special => Stat::SpecialAttack,
            _ => Stat::Attack,
        }, context.critical);
        let attack = context.hooks.attack_modifiers.values().fold(
            attack_stat as f64, |atk, func| atk * func(context)).trunc();
        let defense_stat = self.stat(match context.cls {
            moves::DamageClass::Special => Stat::SpecialDefense,
            _ => Stat::Defense,
        }, context.critical);
        let defense = context.hooks.defense_modifiers.values().fold(
            defense_stat as f64, |def, func| def * func(self, context)).trunc();
        let modifier = context.hooks.damage_modifiers.values().fold(
            1.0, |modi, func| modi * func(self, context));
        let max = (((((((level_factor as f64) * power) * attack)
            / defense).trunc() / 50.0).trunc() + 2.0) * modifier).trunc();
        max.min(std::u16::MAX as f64) as u16
    }

    pub fn direct_damage(&mut self, dmg: u16) {
        self.perm.hp = self.perm.hp.checked_sub(dmg).unwrap_or(0);
    }

    pub fn damage<R: rand::Rng>(
        &mut self, context: &MoveContext, rng: &mut R
    ) -> u16 {
        let max = self.calc_max_damage(context);
        let dmg = ((max * rng.gen_range(85, 101)) / 100).max(1);
        self.direct_damage(dmg);
        dmg
    }
}
