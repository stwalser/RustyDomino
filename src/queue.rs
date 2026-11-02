use crate::queue::QueueError::MoveNotPossibleError;
use crate::queue::Side::{Left, Right};
use crate::stone::{PlayedStone, Stone};
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum QueueError {
    #[error("This stone cannot be played! ")]
    MoveNotPossibleError,
    #[error("You need to play a double as your first move! ")]
    DoubleNecessary,
    #[error(
        "You need to play your highest double first! If you don't have a double, the stone with the highest dot sum! "
    )]
    HighestDouble,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Playable {
    Left,
    Right,
    Both,
}

#[derive(Error, Debug)]
#[error("This value cannot be converted to a Side")]
pub struct SideConversionError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

impl TryFrom<Playable> for Side {
    type Error = SideConversionError;

    fn try_from(value: Playable) -> Result<Self, Self::Error> {
        match value {
            Playable::Left => Ok(Left),
            Playable::Right => Ok(Right),
            Playable::Both => Err(SideConversionError {}),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Queue {
    stones: VecDeque<PlayedStone>,
    expected_start_stone: Option<Stone>,
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            stones: VecDeque::new(),
            expected_start_stone: None,
        }
    }

    pub fn add(&mut self, stone: Stone, side: Side) {
        if self.stones.is_empty() {
            let dots = stone.get_dots();
            self.stones
                .push_back(stone.into_played_stone_by_right(dots[0])); // it is a double. It doesn't matter
        } else {
            match side {
                Side::Left => {
                    let right = self.stones.front().unwrap().get_left();
                    self.stones
                        .push_front(stone.into_played_stone_by_right(right));
                }
                Side::Right => {
                    let left = self.stones.back().unwrap().get_right();
                    self.stones.push_back(stone.into_played_stone_by_left(left));
                }
            }
        }
    }

    pub fn is_playable(&self, stone: Stone) -> Result<Playable, QueueError> {
        if self.stones.is_empty() {
            if !stone.is_double() {
                return Err(QueueError::DoubleNecessary);
            } else if stone != self.expected_start_stone.unwrap() {
                return Err(QueueError::HighestDouble);
            }
            return Ok(Playable::Left);
        }

        let can_left = stone
            .get_dots()
            .contains(&self.stones.front().unwrap().get_left());
        let can_right = stone
            .get_dots()
            .contains(&self.stones.back().unwrap().get_right());

        match (can_left, can_right) {
            (true, true) => Ok(Playable::Both),
            (true, false) => Ok(Playable::Left),
            (false, true) => Ok(Playable::Right),
            (false, false) => Err(MoveNotPossibleError),
        }
    }

    pub fn set_expected_start_stone(&mut self, stone: Stone) {
        self.expected_start_stone = stone.into();
    }

    pub fn open_end_sum(&self) -> u8 {
        self.stones.front().unwrap().get_left() + self.stones.back().unwrap().get_right()
    }
}

impl Display for Queue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "----------")?;
        for stone in &self.stones {
            write!(f, "{}", stone)?;
            write!(f, " ")?;
        }
        writeln!(f)?;
        write!(f, "----------")?;
        Ok(())
    }
}
