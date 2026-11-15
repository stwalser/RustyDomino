use async_trait::async_trait;
use crate::common::protocol::{ClientAnswer, GameCommand};

pub mod local;
pub mod remote;
pub mod client;

#[async_trait]
pub trait Receiver: std::fmt::Debug + Send {
    async fn receive_command(&mut self) -> GameCommand;
    async fn respond(&mut self, client_answer: ClientAnswer);
}
