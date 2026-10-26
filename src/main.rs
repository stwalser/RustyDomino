mod game;
mod player;
mod queue;
mod stone;

use crate::game::{Game, GameError};
use crate::player::Player;
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
    loop {
        print_current_round(&mut game);
        move_and_clear(3 + 1 + game.get_n_stones_of_current_player() + 1 + 1 + 2);
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
    game.print_round_greeting(); // 1 line
    wait_for_press_of(Enter); // 1 line

    let mut drew_two_stones = false;
    loop {
        game.print_current_player_stones(); // x lines

        let mut wrong_input_message = "".to_string();
        let mut input_string = String::new();

        if game.current_player_can_make_a_move() {
            println!("Which stone do you want to play?"); // 1 line
            io::stdin().read_line(&mut input_string).unwrap(); // 1 line
            let input = input_string.trim().to_string();
            if let Ok(n) = input.parse::<usize>() {
                match game.play_stone(n - 1) {
                    Ok(()) => {
                        break;
                    }
                    Err(GameError::Queue(e)) => {
                        wrong_input_message = e.to_string();
                    }
                    Err(e) => {
                        wrong_input_message = e.to_string();
                    }
                }
            } else {
                wrong_input_message = "Enter a number to choose a stone! ".to_string();
            }
        } else {
            if drew_two_stones {
                println!("You still cannot make a move. Press 'enter' to confirm.");
                wait_for_press_of(Enter);
                return;
            }
            println!("You cannot make a move. Press 'd' for drawing two stones from the talon.");
            io::stdin().read_line(&mut input_string).unwrap(); // 1 line
            let input = input_string.trim().to_string();
            if input == "d" {
                if let Err(_) = game.draw_for_current_player() {};
                drew_two_stones = true;
            } else {
                wrong_input_message = "".to_string();
            }
        }

        move_and_clear(
            1 + 1 + game.get_n_stones_of_current_player() - if drew_two_stones { 2 } else { 0 } + 1,
        ); // the player has one store more than printed already
        println!("{}", wrong_input_message)
    }
}
