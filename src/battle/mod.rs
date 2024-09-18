// 本mod用于处理遭遇战相关的逻辑

// 顶层trait : Battle

pub(super) mod board;
pub(super) mod concept;
pub(super) mod unit;
pub(super) mod common;
pub mod constant;

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

pub trait Battle<'a> : 
    Debug +
    Serialize + 
    Deserialize<'a> 
{
    fn exe(&mut self, cmd : impl BattleCommand) -> Result<impl BattleEvent, impl BattleCommandError>;
    fn show(&self) -> impl BattleShow;
}

// 相关接口

pub trait BattleCommand : Debug {

}

pub trait BattleEvent : Debug {

}

pub trait BattleCommandError : Debug {

}

pub trait BattleShow : Debug {

}