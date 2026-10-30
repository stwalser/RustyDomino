use crate::player::{Player, PlayerError};
use crate::queue::{Playable, Queue, QueueError};
use crate::stone::Stone;
use rand::seq::SliceRandom;
use thiserror::Error;

const HIGHEST_NUMBER: u8 = 6;
const DIVISOR: u8 = 3;
static DICTIONARY: &[(usize, usize)] = &[(2, 7), (3, 6), (4, 6)];

#[derive(Debug, Error)]
pub enum GameError {
    #[error(transparent)]
    Queue(#[from] QueueError),
    #[error(transparent)]
    Player(#[from] PlayerError),
}

pub struct Game {
    queue: Queue,
    talon: Vec<Stone>,
    players: Vec<Player>,
    current_player: usize,
}

impl Game {
    pub fn new(players: Vec<Player>) -> Self {
        Self {
            queue: Queue::new(),
            talon: Self::shuffled_stones(),
            players: players.clone(),
            current_player: 0,
        }
    }

    pub fn init_players_stones(&mut self) {
        let n = DICTIONARY[self.players.len() - 2].1;
        for player in &mut self.players {
            player.assign_initial_stones(self.talon.drain(0..n).collect());
        }
    }

    pub fn talon_not_empty(&self) -> bool {
        !self.talon.is_empty()
    }

    pub fn draw_for_current_player(&mut self) {
        self.players[self.current_player].assign_new_stone(self.talon.pop().unwrap())
    }

    pub fn print_board(&self) {
        println!("{}", self.queue);
    }

    pub fn play_stone(&mut self, n: usize, playable: Playable) -> Result<(), GameError> {
        let player = &mut self.players[self.current_player];
        let stone = *player.get_stone(n)?;
        self.queue.add(stone, playable)?; // anderes Feld, kein Konflikt
        player.remove_stone(n); // nur bei Erfolg
        Ok(())
    }

    pub fn check_stone(&mut self, n: usize) -> Result<Playable, GameError> {
        let player = &mut self.players[self.current_player];
        let stone = *player.get_stone(n)?;
        Ok(self.queue.is_playable(stone)?)
    }

    pub fn determine_start_player(&mut self) {
        let (start_player, start_stone) = self
            .players
            .iter()
            .enumerate()
            .filter_map(|(index, player)| player.get_max_double().map(|stone| (index, stone)))
            .max_by_key(|&(_, stone)| stone.double_value())
            .unwrap_or_else(|| {
                self.players
                    .iter()
                    .enumerate()
                    .filter_map(|(index, player)| {
                        player.get_max_dot_sum_stone().map(|stone| (index, stone))
                    })
                    .max_by_key(|&(_, stone)| stone.dot_sum())
                    .unwrap()
            });
        self.current_player = start_player;
        self.queue.set_expected_start_stone(*start_stone);
    }

    fn get_current_player(&self) -> &Player {
        &self.players[self.current_player]
    }

    fn get_current_player_mut(&mut self) -> &mut Player {
        &mut self.players[self.current_player]
    }

    pub fn print_round_greeting(&self) {
        println!(
            "It's your turn {}. Press 'enter' when you're ready.",
            self.get_current_player()
        );
    }

    pub fn print_current_player_stones(&self) {
        self.get_current_player().print_stones();
    }

    pub fn current_player_can_make_a_move(&self) -> bool {
        self.get_current_player().can_play_stone(&self.queue)
    }

    pub fn next_player(&mut self) {
        self.current_player = (self.current_player + 1) % self.players.len()
    }

    pub fn current_player_won(&self) -> bool {
        self.get_current_player().get_n_stones() == 0
    }

    pub fn add_bonus_points(&mut self) {
        let points = self
            .players
            .iter()
            .map(|player| {
                if player != self.get_current_player() {
                    player.get_n_stones()
                } else {
                    0
                }
            })
            .sum::<usize>();
        self.get_current_player_mut().add_points(points as u8);
    }

    pub fn print_points(&self) {
        println!("Congratulations {}! You won!", self.get_current_player());
        println!("Points");
        for player in &self.players {
            println!("{} - {}", player.get_points(), player)
        }
    }

    pub fn check_for_points(&mut self) {
        let open_end_sum = self.queue.open_end_sum();
        if open_end_sum % DIVISOR == 0 {
            self.get_current_player_mut()
                .add_points(open_end_sum / DIVISOR);
        }
    }

    fn shuffled_stones() -> Vec<Stone> {
        let mut stones: Vec<Stone> = (0..=HIGHEST_NUMBER)
            .flat_map(|i| (0..=i).map(move |j| Stone::new(i, j)))
            .collect();
        stones.shuffle(&mut rand::rng());
        stones
    }
}
