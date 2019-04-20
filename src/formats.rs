use crate::battle;
use crate::hooks;
use vdex::Enum;
use enum_repr::EnumRepr;

pub struct SingleBattler<'bat> {
    pub bench: Vec<battle::BenchPokemon>,
    pub current: Option<battle::BattlePokemon<'bat>>,
}

pub struct SingleBattle<'bat> {
    pub hooks: hooks::Hooks,
    pub battler1: SingleBattler<'bat>,
    pub battler2: SingleBattler<'bat>,
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

pub struct LoneDoubleBattler<'bat> {
    pub bench: Vec<battle::BenchPokemon>,
    pub current1: Option<battle::BattlePokemon<'bat>>,
    pub current2: Option<battle::BattlePokemon<'bat>>,
}

pub enum DoubleBattler<'bat> {
    Lone(LoneDoubleBattler<'bat>),
    Pair(SingleBattler<'bat>, SingleBattler<'bat>)
}

pub struct DoubleBattle<'bat> {
    pub hooks: hooks::Hooks,
    pub battler1: DoubleBattler<'bat>,
    pub battler2: DoubleBattler<'bat>,
}
