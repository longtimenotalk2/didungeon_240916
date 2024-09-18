use crate::battle::{common::Pos, unit::Unit};

use super::Board;

impl Board {
    pub fn insert_unit_data(&mut self, pos : Pos, unit_data : Unit) {
        self.units.insert(pos, unit_data);
    }

    pub fn show_multi_line(&self) -> Vec<String> {
        let mut vec = vec![];
        for (pos, unit) in &self.units {
            vec.push(format!("{} : {}", pos, unit.show_one_line()))
        }
        vec
    }
}