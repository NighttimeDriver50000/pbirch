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

pub struct HookValues<'a, 'b, T: Clone> {
    pub battle_map: BTreeMap<HookKey, T>,
    pub battle_iter: Option<btree_map::Iter<'a, HookKey, T>>,
    pub battle_next: Option<(&'a HookKey, &'a T)>,
    pub overlay_iter: btree_map::Iter<'b, HookKey, T>,
    pub overlay_next: Option<(&'b HookKey, &'b T)>,
}

impl<'a, 'b, T: Clone> HookValues<'a, 'b, T> {
    pub fn new(map: &'b HookMap<T>) -> Self {
        let bm = (*(map.battle.borrow())).clone();
        let mut oi = map.overlay.iter();
        Self {
            battle_map: bm,
            battle_iter: None,
            battle_next: None,
            overlay_iter: oi,
            overlay_next: oi.next(),
        }
    }
}

impl<'a, 'b, T: Clone> Clone for HookValues<'a, 'b, T> {
    fn clone(&self) -> Self {
        Self {
            battle_map: self.battle_map.clone(),
            battle_iter: self.battle_iter.clone(),
            battle_next: self.battle_next.clone(),
            overlay_iter: self.overlay_iter.clone(),
            overlay_next: self.overlay_next.clone(),
        }
    }
}

impl<'a, 'b, T: Clone> Iterator for HookValues<'a, 'b, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if let None = self.battle_iter {
            self.battle_iter = Some(self.battle_map.iter());
            self.battle_next = self.battle_iter.map_or(None, |i| i.next());
        }
        if let Some(bn) = self.battle_next {
            if let Some(on) = self.overlay_next {
                if bn.0 <= on.0 {
                    self.battle_next = self.battle_iter.map_or(None, |i| i.next());
                }
                if on.0 <= bn.0 {
                    self.overlay_next = self.overlay_iter.next();
                }
                Some(if bn.0 < on.0 { bn.1.clone() } else { on.1.clone() })
            } else {
                self.battle_next = self.battle_iter.map_or(None, |i| i.next());
                Some(bn.1.clone())
            }
        } else if let Some(on) = self.overlay_next {
            self.overlay_next = self.overlay_iter.next();
            Some(on.1.clone())
        } else {
            None
        }
    }
}

impl<'a, 'b, T: Clone> std::iter::FusedIterator for HookValues<'a, 'b, T> { }

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

    pub fn values<'a, 'b>(&'b self) -> HookValues<'a, 'b, T> {
        HookValues::new(self)
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
