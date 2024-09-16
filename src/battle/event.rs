use super::board::{Id, Pos};

pub(super) type Event = Vec<EventUnit>;

pub(super) enum EventUnit {
    DashToUnit {id : Id, id_to : Id},
    DashToPos {id : Id, pos : Pos},
}