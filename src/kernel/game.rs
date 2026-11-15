use crate::common::stone::Stone;
use crate::kernel::player::Player;
use crate::kernel::queue::{Queue, Side};
use crate::utils::wait_for_press_of;
use crossterm::cursor::RestorePosition;
use crossterm::event::KeyCode::Enter;
use crossterm::execute;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType::FromCursorDown;
use rand::seq::SliceRandom;
use std::io;

const HIGHEST_NUMBER: u8 = 6;

pub enum GameMode {
    Rounds(u32),
    Points(u32),
}

pub enum System {
    Double6,
    Double9,
}

impl System {
    pub fn divisor(&self) -> u8 {
        match self {
            System::Double6 => 3,
            System::Double9 => 5,
        }
    }

    pub fn n_stones(&self, n_players: usize) -> usize {
        match self {
            System::Double6 => match n_players {
                2 => 7,
                3 => 7,
                4 => 5,
                5 => 4,
                6 => 4,
                7 => 3,
                8 => 3,
                9 => 2,
                _ => unreachable!(),
            },
            System::Double9 => match n_players {
                2 => 13,
                3 => 13,
                4 => 11,
                5 => 9,
                6 => 7,
                7 => 6,
                8 => 6,
                9 => 5,
                _ => unreachable!(),
            },
        }
    }
}

pub struct Game {
    game_mode: GameMode,
    system: System,
    counter: u32,
    queue: Queue,
    talon: Vec<Stone>,
    players: Vec<Player>,
    current_player: usize,
}

impl Game {
    pub fn new(players: Vec<Player>, game_mode: GameMode, system: System) -> Self {
        Self {
            game_mode,
            system,
            counter: 0,
            queue: Queue::new(),
            talon: Self::shuffled_stones(),
            players,
            current_player: 0,
        }
    }

    pub fn reset_game(&mut self) {
        self.talon = Self::shuffled_stones();
        self.queue = Queue::new();
        self.counter = 0;
    }

    pub fn init_players_stones(&mut self) {
        let n = self.system.n_stones(self.players.len());
        for player in &mut self.players {
            player.assign_initial_stones(self.talon.drain(0..n).collect());
        }
    }

    async fn draw_for_current_player(&mut self) {
        let queue_string = self.queue_string();
        self.current_player_mut()
            .ask_player_to_draw(queue_string)
            .await;
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

    async fn handle_move(&mut self) {
        let mut hint = None;
        self.current_player_mut().ask_if_ready().await;
        loop {
            if self.current_player().can_play_stone(&self.queue) {
                loop {
                    let queue_string = self.queue_string();
                    let stone = self
                        .current_player_mut()
                        .ask_for_stone(queue_string, hint.clone())
                        .await;
                    let playable = match self.queue.is_playable(stone) {
                        Ok(p) => p,
                        Err(error) => {
                            hint = error.to_string().into();
                            continue;
                        }
                    };
                    let side = match Side::try_from(playable) {
                        Ok(s) => s,
                        Err(_) => self.current_player_mut().ask_for_side().await,
                    };
                    self.play_stone(stone, side);
                    self.check_for_points();
                    break;
                }
            } else if !self.talon.is_empty() {
                self.draw_for_current_player().await;
                hint = "Now you can make a move".to_string().into();
                continue;
            } else {
                self.no_move_possible().await
            }
            break;
        }
    }

    pub async fn run(&mut self) {
        loop {
            self.init_players_stones();
            self.determine_start_player();

            loop {
                self.handle_move().await;

                if self.current_player().n_of_stones() == 0 {
                    self.add_bonus_points();
                    self.print_points();
                    break;
                } else {
                    self.next_player();
                }
            }

            match self.game_mode {
                GameMode::Rounds(rounds) => {
                    self.counter += 1;
                    if self.counter == rounds {
                        break;
                    }
                    println!("Finished round {}/{}", self.counter, rounds);
                    println!("Press enter to start next round");
                    wait_for_press_of(vec![Enter], true).unwrap();
                }
                GameMode::Points(max_points) => {
                    self.counter = self
                        .players
                        .iter()
                        .map(|player| player.points())
                        .max()
                        .unwrap() as u32;
                    if self.counter >= max_points {
                        break;
                    }
                    println!("The best player has {}/{} points", self.counter, max_points);
                    println!("Press enter to start next round");
                    wait_for_press_of(vec![Enter], true).unwrap();
                }
            }
            self.reset_game();
        }

        self.print_final_points();
        for player in &mut self.players {
            player.stop_game().await
        }
    }

    fn print_final_points(&mut self) {
        execute!(io::stdout(), RestorePosition, Clear(FromCursorDown)).unwrap();
        println!("Congratulations to ");
        for player in self.players_with_most_points() {
            println!("{}", player);
        }
        println!("You won");
        self.print_total_points();
    }

    async fn no_move_possible(&mut self) {
        let queue_string = self.queue_string();
        self.current_player_mut()
            .ask_for_no_move_possible_confirmation(queue_string)
            .await;
    }

    fn next_player(&mut self) {
        self.current_player = (self.current_player + 1) % self.players.len()
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
        execute!(io::stdout(), RestorePosition, Clear(FromCursorDown)).unwrap();
        println!(
            "Congratulations {}! You won this round!",
            self.current_player()
        );
        self.print_total_points();
    }

    fn print_total_points(&self) {
        println!("Total Points");
        for player in &self.players {
            println!("{} - {}", player.points(), player)
        }
    }

    fn check_for_points(&mut self) {
        let open_end_sum = self.queue.open_end_sum();
        let divisor = self.system.divisor();
        if open_end_sum.is_multiple_of(divisor) {
            self.current_player_mut().add_points(open_end_sum / divisor);
        }
    }

    fn players_with_most_points(&self) -> Vec<&Player> {
        let max = self
            .players
            .iter()
            .max_by_key(|player| player.points())
            .unwrap();
        self.players
            .iter()
            .filter(|player| player.points() == max.points())
            .collect()
    }

    fn shuffled_stones() -> Vec<Stone> {
        let mut stones: Vec<Stone> = (0..=HIGHEST_NUMBER)
            .flat_map(|i| (0..=i).map(move |j| Stone::new(i, j)))
            .collect();
        stones.shuffle(&mut rand::rng());
        stones
    }
}
