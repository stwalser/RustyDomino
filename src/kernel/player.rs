use crate::common::protocol::{ClientAnswer, GameCommand};
use crate::common::stone::Stone;
use crate::kernel::communicator::Communicator;
use crate::kernel::queue::{Queue, Side};
use std::fmt::{Display, Formatter};

pub enum PlayerMode {
    Local,
    Remote
}

#[derive(Debug)]
pub struct Player {
    name: String, //TODO: add requirement that it is unique
    stones: Vec<Stone>,
    points: u8,
    communicator: Box<dyn Communicator>,
}

impl Player {
    pub fn new(name: String, communicator: Box<dyn Communicator>) -> Self {
        Self {
            name,
            stones: Vec::new(),
            points: 0,
            communicator,
        }
    }

    pub fn assign_initial_stones(&mut self, stones: Vec<Stone>) {
        self.stones = stones;
    }

    pub fn ask_if_ready(&self) {
        let answer = self
            .communicator
            .send_command(GameCommand::CheckIfPlayerReady(self.name.clone()));
        let ClientAnswer::PlayerReady = answer else {
            panic!("The client is going havoc!")
        };
    }

    pub fn ask_for_stone(&self, queue: String, hint: Option<String>) -> Stone {
        let answer = self.communicator.send_command(GameCommand::MakeAMove(
            queue,
            hint,
            self.stones.clone(),
        ));
        let ClientAnswer::PlayStone(stone) = answer else {
            panic!("The client is going havoc!")
        };
        stone
    }

    pub fn ask_for_side(&self) -> Side {
        let answer = self.communicator.send_command(GameCommand::ChooseASide);
        let ClientAnswer::SideChosen(side) = answer else {
            panic!("The client is going havoc!")
        };
        side
    }

    pub fn ask_player_to_draw(&self, queue: String) {
        let answer = self
            .communicator
            .send_command(GameCommand::DrawAStone(queue, self.stones.clone()));
        let ClientAnswer::DrawStone = answer else {
            panic!("The client is going havoc!")
        };
    }

    pub fn ask_for_no_move_possible_confirmation(&self, queue: String) {
        let answer = self
            .communicator
            .send_command(GameCommand::NoMovePossible(queue, self.stones.clone()));
        let ClientAnswer::NoMoveAccepted = answer else {
            panic!("The client is going havoc!")
        };
    }

    pub fn stop_game(&self) {
        let _ = self.communicator.send_command(GameCommand::Stop);
    }

    pub fn assign_new_stone(&mut self, stone: Stone) {
        self.stones.push(stone);
    }

    pub fn remove_stone(&mut self, stone: Stone) {
        self.stones.remove(
            self.stones
                .iter()
                .position(|x| *x == stone)
                .expect("Stone not found in players stones"),
        );
    }

    pub fn max_double(&self) -> Option<&Stone> {
        self.stones
            .iter()
            .filter(|stone| stone.is_double())
            .max_by_key(|stone| stone.double_value())
    }

    pub fn max_dot_sum_stone(&self) -> Option<&Stone> {
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

    pub fn n_of_stones(&self) -> usize {
        self.stones.len()
    }

    pub fn add_points(&mut self, points: u8) {
        self.points += points
    }

    pub fn points(&self) -> u8 {
        self.points
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn remaining_dot_sum(&self) -> u8 {
        self.stones.iter().map(|stone| stone.dot_sum()).sum()
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
