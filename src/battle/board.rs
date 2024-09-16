use std::collections::BTreeMap;

use rand::Rng;

type Pos = i8;

pub struct Board<R : Rng> {
    units : BTreeMap<Pos, UnitData>,
    rng : R,
}

struct UnitData {

}