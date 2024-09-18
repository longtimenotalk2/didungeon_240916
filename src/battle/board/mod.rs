mod dash;
mod basic;
#[cfg(test)]
pub(super) mod test;
mod turn;

use std::collections::BTreeMap;

use num_rational::Rational32;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use super::{common::Pos, unit::Unit};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Board {
    units : BTreeMap<Pos, Unit>,
    turn : i32,
    av : i32,
}