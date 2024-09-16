use super::{Board, Id, Pos, UnitData};

impl UnitData {
    pub(super) fn new_with_name_and_id(name : &str, id : Id) -> Self {
        let mut unit = Self::default();
        unit.name = name.to_string();
        unit.id = id;
        unit
    }
    
    pub(super) fn show_one_line(&self) -> String {
        self.name.clone()
    }
}

impl Board {
    pub(super) fn insert_unit_data(&mut self, pos : Pos, unit_data : UnitData) {
        self.units.insert(pos, unit_data);
    }

    pub(super) fn show_multi_line(&self) -> Vec<String> {
        let mut vec = vec![];
        for (pos, unit) in &self.units {
            vec.push(format!("{} : {}", pos, unit.show_one_line()))
        }
        vec
    }
}