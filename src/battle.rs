// 本mod用于处理遭遇战相关的逻辑

// 顶层trait : Battle

use serde::{Deserialize, Serialize};

pub trait Battle : 
    Debug +
    Serialize + 
    Deserialize 
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