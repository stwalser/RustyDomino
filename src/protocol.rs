use crate::queue::Side;
use crate::stone::Stone;

#[derive(Debug, PartialEq)]
pub enum GameCommand {
    CheckIfPlayerReady(String),
    MakeAMove(String, Option<String>, Vec<Stone>),
    ChooseASide,
    DrawAStone(String, Vec<Stone>),
    NoMovePossible(String, Vec<Stone>),
    Stop
}

#[derive(Debug, PartialEq)]
pub enum ClientAnswer {
    PlayerReady,
    PlayStone(Stone),
    SideChosen(Side),
    DrawStone,
    NoMoveAccepted,
}
