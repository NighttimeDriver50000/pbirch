use crate::battle;
use std::collections::BTreeMap;
use vdex::Ability;
use vdex::moves;
use vdex::items;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum HookSource {
    Engine(u16),
    Move(moves::MoveId),
    Item(items::ItemId),
    Ability(Ability),
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct HookKey {
    pub priority: i8,
    pub source: HookSource,
    pub index: u8,
}

pub struct Hooks {
    pub power_modifiers: BTreeMap<HookKey, fn(&battle::MoveContext) -> f64>,
}
