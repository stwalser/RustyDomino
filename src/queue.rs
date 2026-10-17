use crate::stone::{PlayedStone, Stone};
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum QueueError {
    #[error("This stone cannot be played")]
    MoveNotPossibleError,
    #[error("You need to play a double as your first move")]
    DoubleNecessary,
}

#[derive(Debug, Clone)]
pub struct Queue {
    stones: Vec<PlayedStone>,
}

impl Queue {
    pub fn new() -> Queue {
        Queue { stones: Vec::new() }
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
                return Err(QueueError::DoubleNecessary)
            }
            let dots = stone.get_dots();
            self.stones.push(stone.into_played_stone(dots[0]))
        }

        Ok(())
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
