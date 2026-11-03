use crate::player::Player;
use crate::queue::{Queue, Side};
use crate::stone::Stone;
use rand::seq::SliceRandom;

const HIGHEST_NUMBER: u8 = 6;
const DIVISOR: u8 = 3;
static DICTIONARY: &[(usize, usize)] = &[(2, 7), (3, 6), (4, 6)];

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
            players,
            current_player: 0,
        }
    }

    pub fn refill_talon(&mut self) {
        self.talon = Self::shuffled_stones()
    }

    pub fn init_players_stones(&mut self) {
        let n = DICTIONARY[self.players.len() - 2].1;
        for player in &mut self.players {
            player.assign_initial_stones(self.talon.drain(0..n).collect());
        }
    }

    fn draw_for_current_player(&mut self) {
        self.current_player()
            .ask_player_to_draw(self.queue_string());
        let new_stone = self.talon.pop().unwrap();
        self.current_player_mut().assign_new_stone(new_stone);
    }

    fn queue_string(&self) -> String {
        self.queue.to_string()
    }

    fn play_stone(&mut self, stone: Stone, side: Side) {
        self.queue.add(stone, side);
        self.current_player_mut().remove_stone(stone);
    }

    fn determine_start_player(&mut self) {
        let (start_player, start_stone) = self
            .players
            .iter()
            .enumerate()
            .filter_map(|(index, player)| player.max_double().map(|stone| (index, stone)))
            .max_by_key(|&(_, stone)| stone.double_value())
            .unwrap_or_else(|| {
                self.players
                    .iter()
                    .enumerate()
                    .filter_map(|(index, player)| {
                        player.max_dot_sum_stone().map(|stone| (index, stone))
                    })
                    .max_by_key(|&(_, stone)| stone.dot_sum())
                    .unwrap()
            });
        self.current_player = start_player;
        self.queue.set_expected_start_stone(*start_stone);
    }

    fn current_player(&self) -> &Player {
        &self.players[self.current_player]
    }

    fn current_player_mut(&mut self) -> &mut Player {
        &mut self.players[self.current_player]
    }

    fn handle_move(&mut self) {
        let mut hint = None;
        self.current_player().ask_if_ready();
        loop {
            if self.current_player().can_play_stone(&self.queue) {
                loop {
                    let stone = self
                        .current_player()
                        .ask_for_stone(self.queue_string(), hint.clone());
                    let playable = match self.queue.is_playable(stone) {
                        Ok(p) => p,
                        Err(error) => {
                            hint = error.to_string().into();
                            continue;
                        }
                    };
                    let side = match Side::try_from(playable) {
                        Ok(s) => s,
                        Err(_) => self.current_player().ask_for_side(),
                    };
                    self.play_stone(stone, side);
                    self.current_player().report_stone_played(stone);
                    self.check_for_points();
                    break;
                }
            } else if !self.talon.is_empty() {
                self.draw_for_current_player();
                hint = "Now you can make a move".to_string().into();
                continue;
            } else {
                self.no_move_possible()
            }
            break;
        }
    }

    pub fn run(&mut self) {
        self.init_players_stones();
        self.determine_start_player();

        loop {
            self.handle_move();

            if self.current_player_won() {
                self.add_bonus_points();
                self.print_points();
                break;
            } else {
                self.next_player();
            }
        }
    }

    fn no_move_possible(&self) {
        self.current_player()
            .ask_for_no_move_possible_confirmation(self.queue_string());
    }

    fn next_player(&mut self) {
        self.current_player = (self.current_player + 1) % self.players.len()
    }

    fn current_player_won(&self) -> bool {
        self.current_player().n_of_stones() == 0
    }

    fn add_bonus_points(&mut self) {
        let points = self
            .players
            .iter()
            .map(|player| {
                if player.name() != self.current_player().name() {
                    player.remaining_dot_sum()
                } else {
                    0
                }
            })
            .sum::<u8>();
        self.current_player_mut().add_points(points);
    }

    fn print_points(&self) {
        println!(
            "Congratulations {}! You won this round!",
            self.current_player()
        );
        println!("Total Points");
        for player in &self.players {
            println!("{} - {}", player.points(), player)
        }
        match self.players.iter().max_by_key(|player| player.points()) {
            Some(leader) => {
                println!("Overall leader: {}", leader);
            }
            None => {
                println!(
                    "There is no leader right now. Multiple players have the same number of points."
                )
            }
        }
    }

    fn check_for_points(&mut self) {
        let open_end_sum = self.queue.open_end_sum();
        if open_end_sum.is_multiple_of(DIVISOR) {
            self.current_player_mut()
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

impl Drop for Game {
    fn drop(&mut self) {
        for player in &self.players {
            player.stop_game()
        }
    }
}
