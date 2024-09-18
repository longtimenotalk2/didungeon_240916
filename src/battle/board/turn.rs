use crate::battle::{concept::OrderToken, constant::{AV_EACH_TURN, TIME_EACH_TURN}, unit::{attribute::Attribute, basic::UnitBasic}};

use super::Board;

pub(super) trait Turn {
    fn order_token_list(&self) -> Vec<(i32, OrderToken)>;
    fn next_turn(&mut self) -> OrderToken;
}

impl Turn for Board {
    fn order_token_list(&self) -> Vec<(i32, OrderToken)> {
        let mut list = vec![];
        list.push((self.av, OrderToken::TurnStart));
        for (_, unit) in &self.units {
            let spd = unit.spd();
            let av = TIME_EACH_TURN / spd;
            list.push((av, OrderToken::Unit(unit.id())));
        }
        list.sort_by_key(|(av, _)| *av);
        list
    }

    fn next_turn(&mut self) -> OrderToken {
        match self.order_token_list().remove(0) {
            (_, OrderToken::TurnStart) => {
                self.av = AV_EACH_TURN;
                OrderToken::TurnStart
            },
            (av, ot) => {
                self.av -= av;
                ot
            }
        }
    }
}
