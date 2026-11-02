use crate::queue::Side;
use crate::stone::Stone;

#[derive(Debug, PartialEq)]
pub enum GameCommand {
    InitialStones(Vec<Stone>),
    CheckIfPlayerReady,
    MakeAMove(String, Option<String>),
    ChooseASide,
    StoneAccepted(Stone),
    DrawAStone(String),
    NewStone(Stone),
    NoMovePossible(String),
    Stop
}

#[derive(Debug, PartialEq)]
pub enum ClientAnswer {
    InitialStonesReceived,
    PlayerReady,
    PlayStone(Stone),
    SideChosen(Side),
    StoneRemoved,
    DrawStone,
    StoneAdded,
    NoMoveAccepted,
}
