use crate::ailments;
use crate::hooks;
use crate::team;
use vdex::moves;

pub struct BenchPokemon {
    pub base: team::TeamMember,
    pub status: ailments::BenchAilments,
    pub hp: u16,
}

pub struct BattlePokemon<'bat> {
    pub index: usize,
    pub perm: &'bat mut BenchPokemon,
    pub overlay: team::TeamMember,
    pub status: ailments::BattlerAilments,
    pub stat_changes: [i8; moves::CHANGEABLE_STATS],
}

pub struct MoveContext<'bat> {
    pub mov: &'static moves::Move,
    pub user: &'bat BattlePokemon<'bat>,
    pub hooks: &'bat hooks::Hooks,
    pub bench: &'bat Vec<BenchPokemon>,
}

impl<'bat> BattlePokemon<'bat> {
    pub fn calc_damage(&self, context: &MoveContext) -> u16 {
        let power = context.hooks.power_modifiers.values().fold(
            context.mov.power as f64, |pow, func| pow * func(context));
        0
    }

    pub fn direct_damage(&mut self, dam: u16) {
        self.perm.hp = self.perm.hp.checked_sub(dam).unwrap_or(0);
    }

    pub fn damage(&mut self, context: &MoveContext) -> u16 {
        let dam = self.calc_damage(context);
        self.direct_damage(dam);
        dam
    }
}
