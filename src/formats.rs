use crate::battle;
use crate::hooks::Hooks;
use crate::team::Team;
use std::cell::RefCell;
use std::rc::Rc;
use vdex::Enum;
use enum_repr::EnumRepr;

pub struct SingleBattler {
    pub bench: battle::Bench,
    pub current: battle::Current,
}

impl SingleBattler {
    pub fn new(team: &Team, hooks: &Hooks) -> Self {
        let mut bench = Vec::new();
        for member in team {
            bench.push(Rc::new(RefCell::new(
                battle::BenchPokemon::new(member))));
        }
        let current = Rc::new(RefCell::new(
            battle::BattlePokemon::new(0, &bench[0], hooks)));
        Self { bench, current }
    }
}

pub struct SingleBattle {
    pub hooks: Hooks,
    pub battler1: SingleBattler,
    pub battler2: SingleBattler,
}

impl SingleBattle {
    pub fn new(team1: &Team, team2: &Team) -> Self {
        let hooks = Hooks::new_battle();
        let battler1 = SingleBattler::new(team1, &hooks);
        let battler2 = SingleBattler::new(team2, &hooks);
        Self { hooks, battler1, battler2 }
    }
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

pub struct LoneDoubleBattler {
    pub bench: battle::Bench,
    pub current1: battle::Current,
    pub current2: battle::Current,
}

pub enum DoubleBattler {
    Lone(LoneDoubleBattler),
    Pair(SingleBattler, SingleBattler)
}

pub struct DoubleBattle {
    pub hooks: Hooks,
    pub battler1: DoubleBattler,
    pub battler2: DoubleBattler,
}
