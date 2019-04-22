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
    pub fn new(position: AbsoluteTarget, team: &Team, hooks: &Hooks) -> Self {
        let mut bench = Vec::new();
        for member in team {
            bench.push(Rc::new(RefCell::new(
                battle::BenchPokemon::new(member))));
        }
        let current = Rc::new(RefCell::new(
            battle::BattlePokemon::new(position, 0, &bench[0], hooks)));
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
        let battler1 = SingleBattler::new(
            AbsoluteTarget::Battler1_1, team1, &hooks);
        let battler2 = SingleBattler::new(
            AbsoluteTarget::Battler2_1, team2, &hooks);
        Self { hooks, battler1, battler2 }
    }
}

#[EnumRepr(type = "u8")]
pub enum AbsoluteTarget {
    Battler1_1 = 0,
    Battler1_2,
    Battler2_1,
    Battler2_2,
}

impl AbsoluteTarget {
    pub fn relative(self, user: &AbsoluteTarget) -> RelativeTarget {
        RelativeTarget::from_repr(self.repr() ^ user.repr()).unwrap()
    }
}

#[EnumRepr(type = "u8")]
pub enum RelativeTarget {
    User = 0,
    Ally,
    OpponentForward,
    OpponentAcross,
}

impl RelativeTarget {
    pub fn absolute(self, user: &AbsoluteTarget) -> AbsoluteTarget {
        AbsoluteTarget::from_repr(self.repr() ^ user.repr()).unwrap()
    }
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
