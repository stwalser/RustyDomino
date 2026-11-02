use crate::protocol::{ClientAnswer, GameCommand};
use crate::queue::Side;
use crate::stone::Stone;
use crossterm::cursor::RestorePosition;
use crossterm::event::KeyCode::{Char, Enter};
use crossterm::event::{Event, KeyCode, read};
use crossterm::execute;
use crossterm::terminal::ClearType::FromCursorDown;
use crossterm::terminal::{Clear, disable_raw_mode, enable_raw_mode};
use std::io;
use std::sync::mpsc::{Receiver, SyncSender};

#[derive(Debug)]
pub struct Client {
    command_receiver: Receiver<GameCommand>,
    answer_sender: SyncSender<ClientAnswer>,
    stones: Vec<Stone>,
    name: String,
}

fn wait_for_press_of(key_codes: Vec<KeyCode>) -> io::Result<KeyCode> {
    enable_raw_mode()?;
    loop {
        match read()? {
            Event::FocusGained => {}
            Event::FocusLost => {}
            Event::Key(key) => {
                if key_codes.contains(&key.code) {
                    disable_raw_mode()?;
                    return Ok(key.code);
                }
            }
            Event::Mouse(_) => {}
            Event::Paste(_) => {}
            Event::Resize(_, _) => {}
        }
    }
}

impl Client {
    pub fn new(
        command_receiver: Receiver<GameCommand>,
        answer_sender: SyncSender<ClientAnswer>,
        stones: Vec<Stone>,
        name: String,
    ) -> Self {
        Self {
            command_receiver,
            answer_sender,
            stones,
            name,
        }
    }

    pub fn stone_exists(&self, n: usize) -> bool {
        self.stones.get(n).is_some()
    }

    pub fn run(&mut self) {
        loop {
            let command = self
                .command_receiver
                .recv()
                .expect("The client lost connection to the game. Maybe the game stopped. Exiting");

            match command {
                GameCommand::InitialStones(stones) => {
                    self.stones = stones;
                    self.answer_sender
                        .send(ClientAnswer::InitialStonesReceived)
                        .unwrap()
                }
                GameCommand::CheckIfPlayerReady => {
                    execute!(io::stdout(), RestorePosition, Clear(FromCursorDown)).unwrap();
                    println!(
                        "It's your turn {}. Press 'enter' when you're ready.",
                        self.name
                    );
                    wait_for_press_of(vec![Enter]).unwrap();
                    self.answer_sender.send(ClientAnswer::PlayerReady).unwrap()
                }
                GameCommand::MakeAMove(queue_string, hint) => {
                    let mut input_feedback = hint.unwrap_or_default();
                    loop {
                        execute!(io::stdout(), RestorePosition, Clear(FromCursorDown)).unwrap();
                        println!("{}", input_feedback);
                        println!("{}", queue_string);
                        self.stones.iter().enumerate().for_each(|(index, stone)| {
                            println!("{}: {}", index + 1, stone);
                        });
                        let mut input_string = String::new();
                        println!("Which stone do you want to play?");
                        io::stdin().read_line(&mut input_string).unwrap();
                        let input = input_string.trim().to_string();
                        let Ok(n) = input.parse::<usize>() else {
                            input_feedback = "Enter a number to choose a stone! ".to_string();
                            continue;
                        };
                        if !self.stone_exists(n - 1) {
                            input_feedback = "You don't have that many stones".to_string();
                            continue;
                        }
                        self.answer_sender
                            .send(ClientAnswer::PlayStone(self.stones[n - 1]))
                            .unwrap();
                        break
                    }
                }
                GameCommand::ChooseASide => {
                    execute!(io::stdout(), RestorePosition, Clear(FromCursorDown)).unwrap();
                    println!("Do you want to play the stone on the (l)eft or the (r)ight side?");
                    let side = match wait_for_press_of(vec![Char('l'), Char('r')]).unwrap() {
                        Char('l') => Side::Left,
                        Char('r') => Side::Right,
                        _ => {
                            panic!("How did this happen?")
                        }
                    };
                    self.answer_sender
                        .send(ClientAnswer::SideChosen(side))
                        .unwrap();
                }
                GameCommand::StoneAccepted(stone) => {
                    self.stones.remove(
                        self.stones
                            .iter()
                            .position(|x| *x == stone)
                            .expect("Stone not found in players stones"),
                    );
                    self.answer_sender.send(ClientAnswer::StoneRemoved).unwrap();
                }
                GameCommand::DrawAStone(queue_string) => {
                    execute!(io::stdout(), RestorePosition, Clear(FromCursorDown)).unwrap();
                    println!("{}", queue_string);
                    self.stones.iter().enumerate().for_each(|(index, stone)| {
                        println!("{}: {}", index + 1, stone);
                    });
                    println!(
                        "You cannot make a move. Press 'd' for drawing a stone from the talon."
                    );
                    wait_for_press_of(vec![Char('d')]).unwrap();
                    self.answer_sender.send(ClientAnswer::DrawStone).unwrap();
                }
                GameCommand::NewStone(stone) => {
                    self.stones.push(stone);
                    self.answer_sender.send(ClientAnswer::StoneAdded).unwrap();
                }
                GameCommand::NoMovePossible(queue_string) => {
                    execute!(io::stdout(), RestorePosition, Clear(FromCursorDown)).unwrap();
                    println!("{}", queue_string);
                    self.stones.iter().enumerate().for_each(|(index, stone)| {
                        println!("{}: {}", index + 1, stone);
                    });
                    println!(
                        "You cannot make a move and the talon is empty. Press 'enter' to confirm"
                    );
                    wait_for_press_of(vec![Enter]).unwrap();
                    self.answer_sender
                        .send(ClientAnswer::NoMoveAccepted)
                        .unwrap()
                }
                GameCommand::Stop => break,
            }
        }
    }
}
