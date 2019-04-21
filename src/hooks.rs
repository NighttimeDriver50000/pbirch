use crate::battle;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::btree_map;
use std::fmt;
use std::rc::Rc;
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
pub struct HookValues<'a, T> {
    pub battle_next: Option<(&'a HookKey, &'a T)>,
    pub battle_iter: btree_map::Iter<'a, HookKey, T>,
    pub overlay_next: Option<(&'a HookKey, &'a T)>,
    pub overlay_iter: btree_map::Iter<'a, HookKey, T>,
}

impl<'a, T> HookValues<'a, T> {
    pub fn new(map: &'a HookMap<T>) -> Self {
        let mut bi = map.battle.borrow().iter();
        let mut oi = map.overlay.iter();
        Self {
            battle_next: bi.next(),
            battle_iter: bi,
            overlay_next: oi.next(),
            overlay_iter: oi,
        }
    }
}

impl<'a, T> Iterator for HookValues<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if let Some(bn) = self.battle_next {
            if let Some(on) = self.overlay_next {
                if bn.0 <= on.0 {
                    self.battle_next = self.battle_iter.next();
                }
                if on.0 <= bn.0 {
                    self.overlay_next = self.overlay_iter.next();
                }
                Some(if bn.0 < on.0 { bn.1 } else { on.1 })
            } else {
                self.battle_next = self.battle_iter.next();
                Some(bn.1)
            }
        } else if let Some(on) = self.overlay_next {
            self.overlay_next = self.overlay_iter.next();
            Some(on.1)
        } else {
            None
        }
    }
}

impl<'a, T> std::iter::FusedIterator for HookValues<'a, T> { }

#[derive(Clone)]
pub struct HookMap<T> {
    pub battle: Rc<RefCell<BTreeMap<HookKey, T>>>,
    pub overlay: BTreeMap<HookKey, T>,
}

impl<T> HookMap<T> {
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

    pub fn values<'a>(&'a self) -> HookValues<'a, T> {
        HookValues::new(self)
    }
}

impl<T> fmt::Debug for HookMap<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HookMap")
    }
}

#[derive(Clone, Debug)]
pub struct Hooks {
    pub power_modifiers: HookMap<fn(&battle::DamageContext) -> f64>,
    pub attack_modifiers: HookMap<fn(&battle::DamageContext) -> f64>,
    pub defense_modifiers: HookMap<fn(&battle::DamageContext) -> f64>,
    pub user_damage_modifiers: HookMap<fn(&battle::DamageContext) -> f64>,
    pub target_damage_modifiers: HookMap<fn(&battle::DamageContext) -> f64>,
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
