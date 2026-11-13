use crate::common::stone::Stone;
use crate::kernel::queue::Side;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GameCommand {
    CheckIfPlayerReady(String),
    MakeAMove(String, Option<String>, Vec<Stone>),
    ChooseASide,
    DrawAStone(String, Vec<Stone>),
    NoMovePossible(String, Vec<Stone>),
    Stop,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ClientAnswer {
    PlayerReady,
    PlayStone(Stone),
    SideChosen(Side),
    DrawStone,
    NoMoveAccepted,
}
