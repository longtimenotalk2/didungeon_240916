use crate::battle::common::Id;

use super::Unit;

impl Unit {
    pub fn new_with_name_and_id(name : &str, id : Id) -> Self {
        let mut unit = Self::default();
        unit.name = name.to_string();
        unit.id = id;
        unit
    }
    
    pub fn show_one_line(&self) -> String {
        self.name.clone()
    }
}