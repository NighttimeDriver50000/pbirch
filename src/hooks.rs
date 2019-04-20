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

type PowerFn = Box<dyn Fn(f64, &battle::MoveContext) -> f64>;

pub struct Hooks {
    pub power: BTreeMap<HookKey, PowerFn>,
}
