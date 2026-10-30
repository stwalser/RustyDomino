use crate::queue::{Queue, QueueError};
use crate::stone::Stone;
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlayerError {
    #[error(transparent)]
    CannotPlayThisStone(#[from] QueueError),
    #[error("You don't have that many stones!!! ")]
    NoSuchStone,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    name: String,
    stones: Vec<Stone>,
    points: u8,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            stones: Vec::new(),
            points: 0,
        }
    }

    pub fn assign_initial_stones(&mut self, stones: Vec<Stone>) {
        self.stones = stones
    }

    pub fn assign_new_stone(&mut self, stone: Stone) {
        self.stones.push(stone);
    }

    pub fn remove_stone(&mut self, index: usize) {
        self.stones.remove(index);
    }

    pub fn print_stones(&self) {
        self.stones.iter().enumerate().for_each(|(index, stone)| {
            println!("{}: {}", index + 1, stone);
        });
    }

    pub fn get_stone(&self, n: usize) -> Result<&Stone, PlayerError> {
        self.stones.get(n).ok_or(PlayerError::NoSuchStone)
    }

    pub fn get_max_double(&self) -> Option<&Stone> {
        self.stones
            .iter()
            .filter(|stone| stone.is_double())
            .max_by_key(|stone| stone.double_value())
    }

    pub fn get_max_dot_sum_stone(&self) -> Option<&Stone> {
        self.stones
            .iter()
            .map(|stone| (stone, stone.dot_sum()))
            .max_by_key(|(_, sum)| *sum)
            .map(|(stone, _)| stone)
    }

    pub fn can_play_stone(&self, queue: &Queue) -> bool {
        self.stones
            .iter()
            .any(|stone| queue.is_playable(*stone).is_ok())
    }

    pub fn get_n_stones(&self) -> usize {
        self.stones.len()
    }

    pub fn add_points(&mut self, points: u8) {
        self.points += points
    }

    pub fn get_points(&self) -> u8 {
        self.points
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
