#[cfg(test)]
mod test;
pub(super) mod basic;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::common::{Dir, Id};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub(super) struct Unit {
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