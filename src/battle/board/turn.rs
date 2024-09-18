use crate::battle::concept::OrderToken;

pub(super) trait Turn {
    fn order_token_list(&self) -> Vec<(f64, OrderToken)>;
    fn next_turn(&mut self);
}
