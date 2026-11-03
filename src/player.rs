use crate::protocol::{ClientAnswer, GameCommand};
use crate::queue::{Queue, Side};
use crate::stone::Stone;
use std::fmt::{Display, Formatter};
use std::sync::mpsc::{Receiver, SyncSender};

#[derive(Debug)]
pub struct Player {
    name: String, //TODO: add requirement that it is unique
    stones: Vec<Stone>,
    points: u8,
    command_sender: SyncSender<GameCommand>,
    answer_receiver: Receiver<ClientAnswer>,
}

impl Player {
    pub fn new(
        name: String,
        command_sender: SyncSender<GameCommand>,
        answer_receiver: Receiver<ClientAnswer>,
    ) -> Self {
        Self {
            name,
            stones: Vec::new(),
            points: 0,
            command_sender,
            answer_receiver,
        }
    }

    pub fn assign_initial_stones(&mut self, stones: Vec<Stone>) {
        self.command_sender
            .send(GameCommand::InitialStones(stones.clone()))
            .unwrap();
        let ClientAnswer::InitialStonesReceived = self.answer_receiver.recv().unwrap() else {
            panic!("The client is going havoc!");
        };
        self.stones = stones;
    }

    pub fn ask_if_ready(&self) {
        self.command_sender
            .send(GameCommand::CheckIfPlayerReady)
            .unwrap();
        let ClientAnswer::PlayerReady = self.answer_receiver.recv().unwrap() else {
            panic!("The client is going havoc!")
        };
    }

    pub fn ask_for_stone(&self, queue: String, hint: Option<String>) -> Stone {
        self.command_sender
            .send(GameCommand::MakeAMove(queue, hint))
            .unwrap();
        let ClientAnswer::PlayStone(stone) = self.answer_receiver.recv().unwrap() else {
            panic!("The client is going havoc!")
        };
        stone
    }

    pub fn ask_for_side(&self) -> Side {
        self.command_sender.send(GameCommand::ChooseASide).unwrap();
        let ClientAnswer::SideChosen(side) = self.answer_receiver.recv().unwrap() else {
            panic!("The client is going havoc!")
        };
        side
    }

    pub fn report_stone_played(&self, stone: Stone) {
        self.command_sender
            .send(GameCommand::StoneAccepted(stone))
            .unwrap();
        let ClientAnswer::StoneRemoved = self.answer_receiver.recv().unwrap() else {
            panic!("The client is going havoc!")
        };
    }

    pub fn ask_player_to_draw(&self, queue: String) {
        self.command_sender
            .send(GameCommand::DrawAStone(queue))
            .unwrap();
        let ClientAnswer::DrawStone = self.answer_receiver.recv().unwrap() else {
            panic!("The client is going havoc!")
        };
    }

    pub fn ask_for_no_move_possible_confirmation(&self, queue: String) {
        self.command_sender
            .send(GameCommand::NoMovePossible(queue))
            .unwrap();
        let ClientAnswer::NoMoveAccepted = self.answer_receiver.recv().unwrap() else {
            panic!("The client is going havoc!")
        };
    }

    pub fn stop_game(&self) {
        self.command_sender.send(GameCommand::Stop).unwrap()
    }

    pub fn assign_new_stone(&mut self, stone: Stone) {
        self.stones.push(stone);
        self.command_sender
            .send(GameCommand::NewStone(stone))
            .unwrap();
        let ClientAnswer::StoneAdded = self.answer_receiver.recv().unwrap() else {
            panic!("The client is going havoc")
        };
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
