use crate::clients::Receiver;
use crate::common::protocol::{ClientAnswer, GameCommand};
use crate::kernel::queue::Side;
use crate::utils::{get_int_from_input, wait_for_press_of};
use crossterm::cursor::RestorePosition;
use crossterm::event::KeyCode::{Char, Enter};
use crossterm::execute;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType::FromCursorDown;
use std::io;

pub struct Client {
    communicator: Box<dyn Receiver>,
}

impl Client {
    pub(crate) fn new(communicator: Box<dyn Receiver>) -> Self {
        Self { communicator }
    }

    pub(crate) async fn run(&mut self) {
        loop {
            let command = self.communicator.receive_command().await;

            match command {
                GameCommand::CheckIfPlayerReady(name) => {
                    execute!(io::stdout(), RestorePosition, Clear(FromCursorDown)).unwrap();
                    println!("It's your turn {}. Press 'enter' when you're ready.", name);
                    wait_for_press_of(vec![Enter], true).unwrap();
                    self.communicator.respond(ClientAnswer::PlayerReady).await
                }
                GameCommand::MakeAMove(queue_string, hint, stones) => {
                    let mut input_feedback = hint.unwrap_or_default();
                    loop {
                        execute!(io::stdout(), RestorePosition, Clear(FromCursorDown)).unwrap();
                        println!("{}", input_feedback);
                        println!("{}", queue_string);
                        stones.iter().enumerate().for_each(|(index, stone)| {
                            println!("{}: {}", index + 1, stone);
                        });
                        let n: usize = get_int_from_input();
                        if !stones.get(n - 1).is_some() {
                            input_feedback = "You don't have that many stones".to_string();
                            continue;
                        }
                        self.communicator
                            .respond(ClientAnswer::PlayStone(stones[n - 1]))
                            .await;
                        break;
                    }
                }
                GameCommand::ChooseASide => {
                    execute!(io::stdout(), RestorePosition, Clear(FromCursorDown)).unwrap();
                    println!("Do you want to play the stone on the (l)eft or the (r)ight side?");
                    let side = match wait_for_press_of(vec![Char('l'), Char('r')], true).unwrap() {
                        Char('l') => Side::Left,
                        Char('r') => Side::Right,
                        _ => {
                            panic!("How did this happen?")
                        }
                    };
                    self.communicator
                        .respond(ClientAnswer::SideChosen(side))
                        .await;
                }
                GameCommand::DrawAStone(queue_string, stones) => {
                    execute!(io::stdout(), RestorePosition, Clear(FromCursorDown)).unwrap();
                    println!("{}", queue_string);
                    stones.iter().enumerate().for_each(|(index, stone)| {
                        println!("{}: {}", index + 1, stone);
                    });
                    println!(
                        "You cannot make a move. Press 'd' for drawing a stone from the talon."
                    );
                    wait_for_press_of(vec![Char('d')], true).unwrap();
                    self.communicator.respond(ClientAnswer::DrawStone).await;
                }
                GameCommand::NoMovePossible(queue_string, stones) => {
                    execute!(io::stdout(), RestorePosition, Clear(FromCursorDown)).unwrap();
                    println!("{}", queue_string);
                    stones.iter().enumerate().for_each(|(index, stone)| {
                        println!("{}: {}", index + 1, stone);
                    });
                    println!(
                        "You cannot make a move and the talon is empty. Press 'enter' to confirm"
                    );
                    wait_for_press_of(vec![Enter], true).unwrap();
                    self.communicator
                        .respond(ClientAnswer::NoMoveAccepted)
                        .await
                }
                GameCommand::Stop => {
                    self.communicator.respond(ClientAnswer::WillStop).await;
                    break
                },
            }
        }
    }
}
