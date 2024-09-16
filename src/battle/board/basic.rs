use super::{Board, Id, Pos, UnitData};

impl Board {


    pub(super) fn get_pos(&self, id : Id) -> Pos {
        for (pos, unit_data) in &self.units {
            if unit_data.id == id {
                return *pos
            }
        }
        panic!("id = {} can not found in board", id)
    }

    // pub(super) fn get_id_from_pos(&self, pos : Pos) -> Option<Id> {
    //     self.units.get(&pos).map(|u| u.id)
    // }
}