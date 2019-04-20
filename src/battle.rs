use crate::ailments;
use crate::hooks;
use crate::team;
use vdex::moves;

pub struct BenchPokemon<'team> {
    pub base: &'team team::TeamMember,
    pub status: ailments::BenchAilments,
    pub hp: u16,
}

pub struct BattlePokemon<'bat, 'team> {
    pub index: usize,
    pub perm: &'bat mut BenchPokemon<'team>,
    pub overlay: team::TeamMember,
    pub status: ailments::BattlerAilments,
    pub stat_changes: [i8; moves::CHANGEABLE_STATS],
}

pub struct MoveContext<'bat, 'team> {
    pub mov: &'static moves::Move,
    pub user: &'bat BattlePokemon<'bat, 'team>,
    pub hooks: &'bat hooks::Hooks,
    pub bench: &'bat Vec<BenchPokemon<'team>>,
}

impl<'bat, 'team> BattlePokemon<'bat, 'team> {
    pub fn calc_damage(&self, context: &MoveContext) -> u16 {
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
