use crate::player::{Player, PlayerError};
use crate::queue::{Queue, QueueError};
use crate::stone::Stone;
use rand::seq::SliceRandom;
use thiserror::Error;

const HIGHEST_NUMBER: u8 = 6;
static DICTIONARY: &[(usize, usize)] = &[(2, 7), (3, 6), (4, 6)];

#[derive(Debug, Error)]
pub enum GameError {
    #[error("All stones have been drawn")]
    NoMoreStones,
    #[error(transparent)]
    Queue(#[from] QueueError),
    #[error(transparent)]
    Player(#[from] PlayerError),
}

pub struct Game {
    queue: Queue,
    talon: Vec<Stone>,
    players: Vec<Player>,
    current_player: usize
}

impl Game {
    pub fn new(players: Vec<Player>) -> Self {
        Self {
            queue: Queue::new(),
            talon: Self::shuffled_stones(),
            players: players.clone(),
            current_player: 0
        }
    }

    pub fn init_players_stones(&mut self) {
        let n = DICTIONARY[self.players.len() - 2].1;
        for player in &mut self.players {
            player.assign_initial_stones(self.talon.drain(0..n).collect());
        }
    }

    pub fn draw_for_current_player(&mut self) -> Result<(), GameError> {
        if let Some(stone) = self.talon.pop() {
            self.players[self.current_player].assign_new_stone(stone);
            Ok(())
        } else {
            Err(GameError::NoMoreStones)
        }
    }

    pub fn print_queue(&self) {
        println!("{}", self.queue);
    }

    pub fn play_stone(&mut self, n: usize) -> Result<(), GameError> {
        let player = &mut self.players[self.current_player];
        let stone = *player.propose_stone(n)?; // Stone ist Copy
        self.queue.add(stone)?;                // anderes Feld, kein Konflikt
        player.remove_stone(n);                // nur bei Erfolg
        Ok(())
    }

    pub fn determine_start_player(&mut self) {
        self.current_player = self.players
            .iter()
            .enumerate()
            .filter_map(|(index, player)| {
                player.get_max_double().map(|stone| (index, stone))
            })
            .max_by_key(|&(_, stone)| stone.double_value())
            .map(|(player, _)| player)
            .unwrap_or_else(|| {
                self.players
                    .iter()
                    .enumerate()
                    .filter_map(|(index, player)| {
                        player.get_max_dot_sum().map(|sum| (index, sum))
                    })
                    .max_by_key(|&(_, sum)| sum)
                    .map(|(index, _)| index)
                    .unwrap()
            })
    }

    pub fn get_current_player(&mut self) -> &mut Player {
        &mut self.players[self.current_player]
    }

    pub fn next_player(&mut self) {
        self.current_player = (self.current_player + 1) % self.players.len()
    }

    fn shuffled_stones() -> Vec<Stone> {
        let mut stones: Vec<Stone> = (0..=HIGHEST_NUMBER)
            .flat_map(|i| (0..=i).map(move |j| Stone::new(i, j)))
            .collect();
        stones.shuffle(&mut rand::rng());
        stones
    }
}
