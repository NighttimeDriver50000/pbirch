use crate::battle;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt;
use std::ops::Try;
use std::rc::Rc;
use vdex::Ability;
use vdex::moves::MoveId;
use vdex::items::ItemId;
use veekun::repr::NeverError;

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
    pub fn new_engine(priority: i8, source: u16, index: u8) -> Self {
        Self { priority, source: HookSource::Engine(source), index }
    }

    pub fn new_move(priority: i8, source: MoveId, index: u8) -> Self {
        Self { priority, source: HookSource::Move(source), index }
    }

    pub fn new_item(priority: i8, source: ItemId, index: u8) -> Self {
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

    pub fn try_fold<A, F, R>(
        &self, init: A, func: F
    ) -> R where F: FnMut(A, T) -> R, R: Try<Ok=A> {
        let mut acc = init;
        let mut b_iter = self.battle.borrow().iter();
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
                    acc = func(acc, if b.0 < o.0 { b.1 } else { o.1 })?;
                } else {
                    b_next = b_iter.next();
                    acc = func(acc, b.1)?;
                }
            } else if let Some(o) = o_next {
                o_next = o_iter.next();
                acc = func(acc, o.1)?;
            } else {
                return acc;
            }
        }
    }

    pub fn fold<A, F>(&self, init: A, func: F) -> A where F: FnMut(A, T) -> A {
        self.try_fold(init,
            move |acc, x| Ok::<A, NeverError>(func(acc, x))).unwrap()
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

#[derive(Clone, Debug)]
pub struct Hooks {
    pub power_modifiers: HookMap<DamageHook>,
    pub attack_modifiers: HookMap<DamageHook>,
    pub defense_modifiers: HookMap<DamageHook>,
    pub user_damage_modifiers: HookMap<DamageHook>,
    pub target_damage_modifiers: HookMap<DamageHook>,
}

impl Hooks {
    pub fn new_battle() -> Self {
        Self {
            power_modifiers: HookMap::new_battle(),
            attack_modifiers: HookMap::new_battle(),
            defense_modifiers: HookMap::new_battle(),
            user_damage_modifiers: HookMap::new_battle(),
            target_damage_modifiers: HookMap::new_battle(),
        }
    }

    pub fn new_overlay(battle: &Hooks) -> Self {
        Self {
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
