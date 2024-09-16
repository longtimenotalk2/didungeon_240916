mod dash;
mod basic;
mod unit;
#[cfg(test)]
pub(super) mod test;

use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub(super) type Pos = i8;
pub(super) type Id = u8;

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Board {
    units : BTreeMap<Pos, UnitData>,
    turn : i32,
    av : i32,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
struct UnitData {
    name : String,
    id : Id,
    is_enemy : bool,
    hp_max : i32,
    hp : i32,
    toughness_max : i32,
    toughness : i32,
    tp : i32,
    is_stand : bool,
    dir : Option<Dir>,
    restrain : bool,
    bound : HashMap<RopePart, Rope>,
    state : HashMap<State, StateDetail>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Dir {
    Left,
    Right,
}

impl Dir {
    fn anti(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    fn from_pos_to_pos(pos0 : Pos, pos1 : Pos) -> Option<Self> {
        if pos1 > pos0 {
            Some(Self::Right)
        } else if pos1 < pos0 {
            Some(Self::Left)
        } else {
            None
        }
    }

    fn sign(&self) -> Pos {
        match self {
            Self::Left => -1,
            Self::Right => 1,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
enum RopePart {
    Wrist,
    WristHang,
    Arm,
    ArmFix,
    Thigh,
    Calf,
    Ankle,
    Connect,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
struct Rope {
    finished : bool,
    process : i32,
    tightness : i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
enum State {
    Stun,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct StateDetail {
    duration : i32,
}