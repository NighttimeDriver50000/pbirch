use crate::battle;
use crate::formats::RelativeTarget;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt;
use std::rc::Rc;
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

impl HookKey {
    pub fn new_engine(priority: i8, source: u16, index: u8) -> Self {
        Self { priority, source: HookSource::Engine(source), index }
    }

    pub fn new_move(priority: i8, source: moves::MoveId, index: u8) -> Self {
        Self { priority, source: HookSource::Move(source), index }
    }

    pub fn new_item(priority: i8, source: items::ItemId, index: u8) -> Self {
        Self { priority, source: HookSource::Item(source), index }
    }

    pub fn new_ability(priority: i8, source: Ability, index: u8) -> Self {
        Self { priority, source: HookSource::Ability(source), index }
    }
}

#[derive(Clone)]
pub struct HookMap<T: Clone> {
    pub battle: Rc<RefCell<BTreeMap<HookKey, T>>>,
    pub overlay: BTreeMap<HookKey, T>,
}

impl<T: Clone> HookMap<T> {
    pub fn new_battle() -> Self {
        Self {
            battle: Rc::new(RefCell::new(BTreeMap::new())),
            overlay: BTreeMap::new(),
        }
    }

    pub fn new_overlay(battle: &Rc<RefCell<BTreeMap<HookKey, T>>>) -> Self {
        Self {
            battle: battle.clone(),
            overlay: BTreeMap::new(),
        }
    }

    pub fn fold<A, F>(
        &self, init: A, mut func: F
    ) -> A where F: FnMut(A, &T) -> A {
        let mut acc = init;
        let battle_borrow = self.battle.borrow();
        let mut b_iter = battle_borrow.iter();
        let mut b_next = b_iter.next();
        let mut o_iter = self.overlay.iter();
        let mut o_next = o_iter.next();
        loop {
            if let Some(b) = b_next {
                if let Some(o) = o_next {
                    if b.0 <= o.0 {
                        b_next = b_iter.next();
                    }
                    if o.0 <= b.0 {
                        o_next = o_iter.next();
                    }
                    acc = func(acc, if b.0 < o.0 { b.1 } else { o.1 });
                } else {
                    b_next = b_iter.next();
                    acc = func(acc, b.1);
                }
            } else if let Some(o) = o_next {
                o_next = o_iter.next();
                acc = func(acc, o.1);
            } else {
                return acc;
            }
        }
    }
}

impl<T: Clone> fmt::Debug for HookMap<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HookMap")
    }
}

#[derive(Copy)]
pub struct DamageHook(pub fn(&battle::DamageContext) -> f64);

impl Clone for DamageHook {
    fn clone(&self) -> Self {
        DamageHook(self.0)
    }
}

#[derive(Copy)]
pub struct TargetingHook(
    pub fn(&battle::Current, moves::Target) -> RelativeTarget);

impl Clone for TargetingHook {
    fn clone(&self) -> Self {
        TargetingHook(self.0)
    }
}

#[derive(Clone)]
pub struct TargetingPair(
    pub Rc<RefCell<TargetingHook>>, pub Option<TargetingHook>);

impl TargetingPair {
    pub fn call(
        &self, user: &battle::Current, mtgt: moves::Target
    ) -> RelativeTarget {
        if let Some(overlay) = self.1 {
            overlay.0(user, mtgt)
        } else {
            self.0.borrow().0(user, mtgt)
        }
    }
}

impl fmt::Debug for TargetingPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TargetingPair")
    }
}

#[derive(Clone, Debug)]
pub struct Hooks {
    pub targeting: TargetingPair,
    pub user_accuracy_modifiers: HookMap<DamageHook>,
    pub target_accuracy_modifiers: HookMap<DamageHook>,
    pub critical_cancels: HookMap<bool>,
    pub power_modifiers: HookMap<DamageHook>,
    pub attack_modifiers: HookMap<DamageHook>,
    pub defense_modifiers: HookMap<DamageHook>,
    pub user_damage_modifiers: HookMap<DamageHook>,
    pub target_damage_modifiers: HookMap<DamageHook>,
}

impl Hooks {
    pub fn new_battle() -> Self {
        Self {
            targeting: TargetingPair(Rc::new(RefCell::new(TargetingHook(|_, mtgt| {
                match mtgt {
                    moves::Target::SpecificMove
                        => panic!("Not implemented yet!"),
                    moves::Target::SelectedPokemonReuseStolen
                        | moves::Target::RandomOpponent
                        | moves::Target::SelectedPokemon
                        => RelativeTarget::OpponentForward,
                    moves::Target::UserOrAlly
                        => RelativeTarget::User,
                    _ => panic!("Only call targeting if the target can vary!"),
                }
            }))), None),
            user_accuracy_modifiers: HookMap::new_battle(),
            target_accuracy_modifiers: HookMap::new_battle(),
            critical_cancels: HookMap::new_battle(),
            power_modifiers: HookMap::new_battle(),
            attack_modifiers: HookMap::new_battle(),
            defense_modifiers: HookMap::new_battle(),
            user_damage_modifiers: HookMap::new_battle(),
            target_damage_modifiers: HookMap::new_battle(),
        }
    }

    pub fn new_overlay(battle: &Hooks) -> Self {
        Self {
            targeting: TargetingPair(battle.targeting.0.clone(), None),
            user_accuracy_modifiers:
                HookMap::new_overlay(&battle.user_accuracy_modifiers.battle),
            target_accuracy_modifiers:
                HookMap::new_overlay(&battle.target_accuracy_modifiers.battle),
            critical_cancels:
                HookMap::new_overlay(&battle.critical_cancels.battle),
            power_modifiers:
                HookMap::new_overlay(&battle.power_modifiers.battle),
            attack_modifiers:
                HookMap::new_overlay(&battle.attack_modifiers.battle),
            defense_modifiers:
                HookMap::new_overlay(&battle.defense_modifiers.battle),
            user_damage_modifiers:
                HookMap::new_overlay(&battle.user_damage_modifiers.battle),
            target_damage_modifiers:
                HookMap::new_overlay(&battle.target_damage_modifiers.battle),
        }
    }
}
