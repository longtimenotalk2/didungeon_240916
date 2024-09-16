mod basic;

use std::collections::{BTreeMap, HashMap, HashSet};

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

type Pos = i8;

// #[derive(Debug, Default)]
pub struct Board<R : Rng + Default + Debug> {
    units : BTreeMap<Pos, UnitData>,
    rng : R,
    turn : i32,
    av : i32,
}

// #[derive(Debug, Serialize, Deserialize, Default)]
struct UnitData {
    name : String,
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
    state : HashSet<StateData>,
}

#[derive(Debug, Serialize, Deserialize)]
enum Dir {
    Left,
    Right,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize, Default)]
struct Rope {
    finished : bool,
    process : i32,
    tightness : i32,
}

#[derive(Debug, Serialize, Deserialize)]
enum StateData {
    Stun {duration : i32},
}