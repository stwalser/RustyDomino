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

    pub fn add(&mut self, stone: Stone) -> Result<(), QueueError> {
        if self.stones.is_empty() {
            if !stone.is_double() {
                return Err(QueueError::DoubleNecessary);
            } else if stone != self.expected_start_stone.unwrap() {
                return Err(QueueError::HighestDouble);
            }
            let dots = stone.get_dots();
            self.stones
                .push_back(stone.into_played_stone_by_right(dots[0])); // it is a double. It doesn't matter
            return Ok(())
        } else {
            if let Some(last_stone) = self.stones.back() {
                if stone.get_dots().contains(&last_stone.get_right()) {
                    let left = last_stone.get_right();
                    self.stones.push_back(stone.into_played_stone_by_left(left));
                    return Ok(());
                } else if let Some(first_stone) = self.stones.front() {
                    if stone.get_dots().contains(&first_stone.get_left()) {
                        let right = first_stone.get_left();
                        self.stones
                            .push_front(stone.into_played_stone_by_right(right));
                        return Ok(());
                    }
                }
            }
        }
        Err(QueueError::MoveNotPossibleError)
    }

    pub fn is_playable(&self, stone: &Stone) -> bool {
        if self.stones.is_empty() {
            return true;
        }
        stone
            .get_dots()
            .contains(&self.stones.back().unwrap().get_right())
            || stone
                .get_dots()
                .contains(&self.stones.front().unwrap().get_left())
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
