use crate::battle;
use std::collections::BTreeMap;
use vdex::Ability;
use vdex::moves::MoveId;
use vdex::items::ItemId;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum HookSource {
    Engine(u16),
    Move(MoveId),
    Item(ItemId),
    Ability(Ability),
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct HookKey {
    pub priority: i8,
    pub source: HookSource,
    pub index: u8,
}

impl HookKey {
    pub fn new_engine(priority: i8, source: u16, index: u8) -> HookKey {
        HookKey { priority, source: HookSource::Engine(source), index }
    }

    pub fn new_move(priority: i8, source: MoveId, index: u8) -> HookKey {
        HookKey { priority, source: HookSource::Move(source), index }
    }

    pub fn new_item(priority: i8, source: ItemId, index: u8) -> HookKey {
        HookKey { priority, source: HookSource::Item(source), index }
    }

    pub fn new_ability(priority: i8, source: Ability, index: u8) -> HookKey {
        HookKey { priority, source: HookSource::Ability(source), index }
    }
}

pub struct Hooks {
    pub power_modifiers: BTreeMap<HookKey, fn(&battle::MoveContext) -> f64>,
    pub attack_modifiers: BTreeMap<HookKey, fn(&battle::MoveContext) -> f64>,
    pub defense_modifiers: BTreeMap<HookKey,
        fn(&battle::BattlePokemon, &battle::MoveContext) -> f64>,
    pub damage_modifiers: BTreeMap<HookKey,
        fn(&battle::BattlePokemon, &battle::MoveContext) -> f64>,
}

impl Hooks {
    pub fn new() -> Hooks {
        let mut hooks = Hooks {
            power_modifiers: BTreeMap::new(),
            attack_modifiers: BTreeMap::new(),
            defense_modifiers: BTreeMap::new(),
            damage_modifiers: BTreeMap::new(),
        };
        hooks.damage_modifiers.insert(HookKey::new_engine(0, 0, 0),
            |_, ctx| if ctx.targets > 1 { 0.75 } else { 1.0 });
        hooks.damage_modifiers.insert(HookKey::new_engine(0, 0, 1),
            |_, ctx| if ctx.critical { 2.0 } else { 1.0 });
        hooks.damage_modifiers.insert(HookKey::new_engine(0, 0, 2),
            |_, ctx| if ctx.user.types.contains(ctx.typ) { 1.5 } else { 1.0 });
        hooks.damage_modifiers.insert(HookKey::new_engine(0, 0, 3),
            |tgt, ctx| tgt.efficacy(ctx.typ));
        hooks
    }
}
