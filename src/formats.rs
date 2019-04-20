use crate::battle;
use crate::hooks;
use vdex::Enum;
use enum_repr::EnumRepr;

pub struct SingleBattler<'bat, 'team> {
    pub bench: Vec<battle::BenchPokemon<'team>>,
    pub current: Option<battle::BattlePokemon<'bat, 'team>>,
}

pub struct SingleBattle<'bat, 'team1, 'team2> {
    pub hooks: hooks::Hooks,
    pub battler1: SingleBattler<'bat, 'team1>,
    pub battler2: SingleBattler<'bat, 'team2>,
}

#[EnumRepr(type = "u8")]
pub enum AbsoluteTarget {
    Opponent1 = 0,
    Opponent2,
    Ally1,
    Ally2,
}

#[EnumRepr(type = "u8")]
pub enum RelativeTarget {
    Opponent1 = 0,
    Opponent2,
    User,
    Ally,
}

pub struct LoneDoubleBattler<'bat, 'team> {
    pub bench: Vec<battle::BenchPokemon<'team>>,
    pub current1: Option<battle::BattlePokemon<'bat, 'team>>,
    pub current2: Option<battle::BattlePokemon<'bat, 'team>>,
}

pub enum DoubleBattler<'bat, 'team1, 'team2> {
    Lone(LoneDoubleBattler<'bat, 'team1>),
    Pair(SingleBattler<'bat, 'team1>, SingleBattler<'bat, 'team2>)
}

pub struct DoubleBattle<'bat, 'team1, 'team2, 'team3, 'team4> {
    pub hooks: hooks::Hooks,
    pub battler1: DoubleBattler<'bat, 'team1, 'team2>,
    pub battler2: DoubleBattler<'bat, 'team3, 'team4>,
}
