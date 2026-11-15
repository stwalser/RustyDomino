use crate::common::protocol::{ClientAnswer, GameCommand};
use crate::common::stone::Stone;
use crate::kernel::communicator::Sender;
use crate::kernel::queue::{Queue, Side};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Player {
    name: String, //TODO: add requirement that it is unique
    stones: Vec<Stone>,
    points: u8,
    communicator: Box<dyn Sender>,
}

impl Player {
    pub fn new(name: String, communicator: Box<dyn Sender>) -> Self {
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

    pub async fn ask_if_ready(&mut self) {
        let answer = self
            .communicator
            .send_command(GameCommand::CheckIfPlayerReady(self.name.clone()))
            .await;
        let ClientAnswer::PlayerReady = answer else {
            panic!("The clients is going havoc!")
        };
    }

    pub async fn ask_for_stone(&mut self, queue: String, hint: Option<String>) -> Stone {
        let answer = self
            .communicator
            .send_command(GameCommand::MakeAMove(queue, hint, self.stones.clone()))
            .await;
        let ClientAnswer::PlayStone(stone) = answer else {
            panic!("The clients is going havoc!")
        };
        stone
    }

    pub async fn ask_for_side(&mut self) -> Side {
        let answer = self
            .communicator
            .send_command(GameCommand::ChooseASide)
            .await;
        let ClientAnswer::SideChosen(side) = answer else {
            panic!("The clients is going havoc!")
        };
        side
    }

    pub async fn ask_player_to_draw(&mut self, queue: String) {
        let answer = self
            .communicator
            .send_command(GameCommand::DrawAStone(queue, self.stones.clone()))
            .await;
        let ClientAnswer::DrawStone = answer else {
            panic!("The clients is going havoc!")
        };
    }

    pub async fn ask_for_no_move_possible_confirmation(&mut self, queue: String) {
        let answer = self
            .communicator
            .send_command(GameCommand::NoMovePossible(queue, self.stones.clone()))
            .await;
        let ClientAnswer::NoMoveAccepted = answer else {
            panic!("The clients is going havoc!")
        };
    }

    pub async fn stop_game(&mut self) {
        self.communicator.send_command(GameCommand::Stop).await;
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
