use crate::common::protocol::{ClientAnswer, GameCommand};
use async_trait::async_trait;

pub mod local;
pub mod remote;

#[async_trait]
pub trait Sender: std::fmt::Debug {
    async fn send_command(&mut self, game_command: GameCommand) -> ClientAnswer;
}
