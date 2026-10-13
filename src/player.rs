use crate::queue::QueueError;
use crate::stone::Stone;
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlayerError {
    #[error(transparent)]
    CannotPlayThisStone(#[from] QueueError),
    #[error("You don't have that many stones!!!")]
    NoSuchStone,
}

#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    id: usize,
    stones: Vec<Stone>,
}

impl Player {
    pub fn new(name: String, id: usize) -> Self {
        Self {
            name,
            id,
            stones: Vec::new(),
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
            println!("{}: {}", index, stone);
        });
    }

    pub fn propose_stone(&self, n: usize) -> Result<&Stone, PlayerError> {
        self.stones.get(n).ok_or(PlayerError::NoSuchStone)
    }

    pub fn get_max_double(&self) -> Option<&Stone> {
        self.stones
            .iter()
            .filter(|stone| stone.is_double())
            .max_by_key(|stone| stone.double_value())
    }

    pub fn get_max_dot_sum(&self) -> Option<u8> {
        self.stones.iter().map(|stone| stone.dot_sum()).max()
    }
    
    pub fn print_round_greeting(&self) {
        println!("It's your turn player {}", self.name);
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
