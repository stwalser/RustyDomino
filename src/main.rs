mod game;
mod player;
mod queue;
mod stone;

use crate::game::{Game, GameError};
use crate::player::Player;
use crate::queue::QueueError;
use crossterm::cursor::MoveUp;
use crossterm::event::KeyCode::Enter;
use crossterm::event::{Event, KeyCode, read};
use crossterm::execute;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType::FromCursorDown;
use std::io;

fn main() {
    println!("Welcome to Domino");
    let n_players = loop {
        println!("How many people will play (2-4)?");
        let mut n_players_string = String::new();
        io::stdin().read_line(&mut n_players_string).unwrap();
        if let Ok(n) = n_players_string.trim().parse::<usize>() {
            if n >= 2 && n <= 4 {
                break n;
            }
            move_and_clear(2);
            print!("{} players are not supported! ", n)
        } else {
            move_and_clear(2);
            print!("Please enter a number! ")
        }
    };

    let players = (0..n_players)
        .map(|id| {
            move_and_clear(2);
            println!("Please enter the name of person {}:", id + 1);
            let mut name_string = String::new();
            io::stdin().read_line(&mut name_string).unwrap();
            Player::new(name_string.trim().parse().unwrap())
        })
        .collect::<Vec<Player>>();

    let mut game = Game::new(players);
    game.init_players_stones();

    game.determine_start_player();

    move_and_clear(2);
    println!(
        "{}: You will begin the game. Press enter when you're ready.",
        game.get_current_player()
    );
    wait_for_press_of(Enter);

    move_and_clear(2);
    loop {
        print_current_round(&mut game);
        // if let Err(_) = game.draw_for_current_player() {}
        move_and_clear(3 + 1 + game.get_current_player().get_n_stones() as u16 + 1 + 2);
        game.next_player();
    }
}

fn move_and_clear(lines: u16) {
    execute!(io::stdout(), MoveUp(lines), Clear(FromCursorDown)).unwrap();
}

fn wait_for_press_of(key_code: KeyCode) {
    loop {
        match read().unwrap() {
            Event::FocusGained => {}
            Event::FocusLost => {}
            Event::Key(key) => {
                if key.code == key_code {
                    return;
                }
            }
            Event::Mouse(_) => {}
            Event::Paste(_) => {}
            Event::Resize(_, _) => {}
        }
    }
}

fn print_current_round(game: &mut Game) {
    game.print_queue(); // 3 lines

    let player = game.get_current_player();

    player.print_round_greeting(); // 1 line
    player.print_stones(); // x lines

    loop {
        println!("Which stone do you want to play?"); // 1 line
        let mut chosen_stone_string = String::new();
        io::stdin().read_line(&mut chosen_stone_string).unwrap(); // 1 line
        if let Ok(n) = chosen_stone_string.trim().parse::<usize>() {
            match game.play_stone(n - 1) {
                Ok(()) => {
                    break;
                }
                Err(GameError::Queue(QueueError::DoubleNecessary)) => {
                    move_and_clear(2);
                    print!("Start with a double! ")
                }
                Err(e) => {
                    move_and_clear(2);
                    print!("{e} ")
                }
            }
        } else {
            move_and_clear(2);
            print!("Enter a number to choose a stone! ")
        }
    }
}
