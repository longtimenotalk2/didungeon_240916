use crate::battle::{board::Dir, event::{Event, EventUnit}};

use super::{Board, Id, Pos};

pub(super) trait BoardDash {
    fn dash_to_unit(&mut self, id : Id, id_to : Id) -> Event;
    fn dash_to_pos(&mut self, id : Id, pos : Pos) -> Event;
}

impl BoardDash for Board {
    fn dash_to_unit(&mut self, id : Id, id_to : Id) -> Event {
        let pos_from = self.get_pos(id);
        let pos_tar = self.get_pos(id_to);
        let dash_dir = Dir::from_pos_to_pos(pos_from, pos_tar).unwrap();
        let pos_to = pos_tar - dash_dir.sign();

        self.dash_to_pos(id, pos_to);

        vec![EventUnit::DashToUnit { id, id_to}]
    }

    fn dash_to_pos(&mut self, id : Id, pos : Pos) -> Event {
        let pos_from = self.get_pos(id);
        let dir_push = Dir::from_pos_to_pos(pos_from, pos).map(|d| d.anti());

        let unit = self.units.remove(&pos_from).unwrap();

        if let Some(dir_push) = dir_push {
            let mut pos_now = pos;
            let mut unit_hanging = self.units.remove(&pos_now);
            for _ in 0..99 {
                if unit_hanging.is_some() {
                    pos_now = pos_now + dir_push.sign();
                    unit_hanging = self.units.insert(pos_now, unit_hanging.take().unwrap());
                }
            }
        }


        self.units.insert(pos, unit);

        vec![EventUnit::DashToPos { id, pos}]
    }
}

#[cfg(test)]
mod test {
    use crate::battle::board::{Board, UnitData};

    use super::BoardDash;

    #[test]
    fn test() {
        let mut board = Board::default();
        let a = UnitData::new_with_name_and_id("A", 0);
        let b = UnitData::new_with_name_and_id("B", 1);
        let c = UnitData::new_with_name_and_id("C", 2);
        let d = UnitData::new_with_name_and_id("D", 3);

        board.insert_unit_data(0, a);
        board.insert_unit_data(1, b);
        board.insert_unit_data(2, c);
        board.insert_unit_data(3, d);

        for line in board.show_multi_line() {
            println!("{}", line);
        }

        board.dash_to_unit(0, 2);

        println!("---------------------------------------------------------");
        for line in board.show_multi_line() {
            println!("{}", line);
        }
    }
}