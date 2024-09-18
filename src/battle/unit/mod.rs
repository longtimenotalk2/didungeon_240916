#[cfg(test)]
mod test;
pub(super) mod basic;
pub(super) mod attribute;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::common::{Dir, Id};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub(super) struct Unit {
    name : String,
    id : Id,
    is_enemy : bool,
    attribute : Attribute,
    hp : i32,
    hurt : i32,
    av : Option<i32>,
    is_stand : bool,
    dir : Option<Dir>,
    bound : HashMap<RopePart, Rope>,
    state : HashMap<State, StateDetail>,
    scuffle : Option<(Id, ScuffleState)>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
struct Attribute {
    hp_max : i32,
    spd : i32,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
enum ScuffleState {
    Win,
    Good,
    Duaw,
    Bad,
    Loose,
}