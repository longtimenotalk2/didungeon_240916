use serde::{Deserialize, Serialize};

pub(super) type Pos = i8;
pub(super) type Id = u8;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(super) enum Dir {
    Left,
    Right,
}

impl Dir {
    pub(super) fn anti(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    pub(super) fn from_pos_to_pos(pos0 : Pos, pos1 : Pos) -> Option<Self> {
        if pos1 > pos0 {
            Some(Self::Right)
        } else if pos1 < pos0 {
            Some(Self::Left)
        } else {
            None
        }
    }

    pub(super) fn sign(&self) -> Pos {
        match self {
            Self::Left => -1,
            Self::Right => 1,
        }
    }
}