use crate::battle;
use crate::hooks::Hooks;
use crate::team::TeamMember;
use std::cell::RefCell;
use std::rc::Rc;
use vdex::Enum;
use enum_repr::EnumRepr;

type Bench = Vec<Rc<RefCell<battle::BenchPokemon>>>;
type Current = Rc<RefCell<battle::BattlePokemon>>;

pub struct SingleBattler {
    pub bench: Bench,
    pub current: Current,
}

impl SingleBattler {
    pub fn new(team: &Vec<Rc<TeamMember>>, hooks: &Hooks) -> Self {
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
    pub bench: Bench,
    pub current1: Current,
    pub current2: Current,
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
