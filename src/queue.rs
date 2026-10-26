use std::cmp::PartialEq;
use crate::stone::{PlayedStone, Stone};
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum QueueError {
    #[error("This stone cannot be played! ")]
    MoveNotPossibleError,
    #[error("You need to play a double as your first move! ")]
    DoubleNecessary,
    #[error("You need to play your highest double first! If you don't have a double, the stone with the highest dot sum! "
    )]
    HighestDouble,
}

#[derive(Debug, Clone)]
pub struct Queue {
    stones: Vec<PlayedStone>,
    expected_start_stone: Option<Stone>,
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            stones: Vec::new(),
            expected_start_stone: None,
        }
    }

    pub fn add(&mut self, stone: Stone) -> Result<(), QueueError> {
        if let Some(last_played) = self.stones.last() {
            if !stone.get_dots().contains(&last_played.get_open()) {
                return Err(QueueError::MoveNotPossibleError);
            }

            let closed = last_played.get_open();
            self.stones.push(stone.into_played_stone(closed))
        } else {
            if !stone.is_double() {
                return Err(QueueError::DoubleNecessary);
            } else if stone != self.expected_start_stone.unwrap() {
                return Err(QueueError::HighestDouble)
            }
            let dots = stone.get_dots();
            self.stones.push(stone.into_played_stone(dots[0]))
        }

        Ok(())
    }

    pub fn is_playable(&self, stone: &Stone) -> bool {
        if self.stones.is_empty() {
            return true;
        }
        stone
            .get_dots()
            .contains(&self.stones.last().unwrap().get_open())
    }

    pub fn set_expected_start_stone(&mut self, stone: Stone) {
        self.expected_start_stone = stone.into();
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
