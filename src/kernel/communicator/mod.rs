use crate::common::protocol::{ClientAnswer, GameCommand};

pub mod local;
pub mod remote;

pub trait Sender: std::fmt::Debug {
    fn send_command(&self, game_command: GameCommand) -> ClientAnswer;
}
