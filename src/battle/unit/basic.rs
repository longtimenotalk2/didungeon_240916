use crate::battle::common::Id;

use super::Unit;

pub(in super::super) trait UnitBasic {
    fn id(&self) -> Id;
}

impl UnitBasic for Unit {
    fn id(&self) -> Id {
        self.id
    }
}