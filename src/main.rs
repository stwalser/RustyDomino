mod game;
mod player;
mod queue;
mod stone;

use crate::game::{Game, GameError};
use crate::player::Player;
use crossterm::cursor::{MoveTo, RestorePosition, SavePosition};
use crossterm::event::KeyCode::{Char, Enter};
use crossterm::event::{Event, KeyCode, read};
use crossterm::execute;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType::FromCursorDown;
use std::io;

fn main() -> io::Result<()> {
    execute!(io::stdout(), MoveTo(0, 0), Clear(FromCursorDown))?;
    println!("---RustyDomino---");
    execute!(io::stdout(), SavePosition)?;
    println!("Welcome to RustyDomino!");
    let n_players = loop {
        println!("How many people will play (2-4)?");
        let mut n_players_string = String::new();
        io::stdin().read_line(&mut n_players_string)?;
        if let Ok(n) = n_players_string.trim().parse::<usize>() {
            if n >= 2 && n <= 4 {
                break n;
            }
            execute!(io::stdout(), RestorePosition, Clear(FromCursorDown))?;
            println!("{} players are not supported! ", n)
        } else {
            execute!(io::stdout(), RestorePosition, Clear(FromCursorDown))?;
            println!("Please enter a number! ")
        }
    };

    let players = (0..n_players)
        .map(|id| {
            execute!(io::stdout(), RestorePosition, Clear(FromCursorDown)).unwrap();
            println!("Please enter the name of person {}:", id + 1);
            let mut name_string = String::new();
            io::stdin().read_line(&mut name_string).unwrap();
            Player::new(name_string.trim().parse().unwrap())
        })
        .collect::<Vec<Player>>();

    let mut game = Game::new(players);
    game.init_players_stones();

    game.determine_start_player();

    execute!(io::stdout(), RestorePosition, Clear(FromCursorDown))?;
    loop {
        game.print_round_greeting();
        wait_for_press_of(Enter);
        execute!(io::stdout(), RestorePosition, Clear(FromCursorDown))?;
        print_current_round(&mut game)?;
        execute!(io::stdout(), RestorePosition, Clear(FromCursorDown))?;
        if game.current_player_won() {
            println!("Congratulations!!! You won!!!");
            break;
        } else {
            game.next_player();
        }
    }
    Ok(())
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

fn print_current_round(game: &mut Game) -> io::Result<()> {
    let mut last_round_feedback = "".to_string();

    loop {
        game.print_board();
        game.print_current_player_stones();

        let mut input_string = String::new();

        println!("{}", last_round_feedback);
        if game.current_player_can_make_a_move() {
            println!("Which stone do you want to play?");
            io::stdin().read_line(&mut input_string)?;
            let input = input_string.trim().to_string();

            if let Ok(n) = input.parse::<usize>() {
                match game.play_stone(n - 1) {
                    Ok(()) => {
                        break;
                    }
                    Err(GameError::Queue(e)) => {
                        last_round_feedback = e.to_string();
                    }
                    Err(e) => {
                        last_round_feedback = e.to_string();
                    }
                }
            } else {
                last_round_feedback = "Enter a number to choose a stone! ".to_string();
            }
        } else if game.talon_not_empty() {
            println!("You cannot make a move. Press 'd' for drawing a stone from the talon.");
            wait_for_press_of(Char('d'));
            game.draw_for_current_player()
        } else {
            println!("You cannot make a move and the talon is empty. Press 'enter' to confirm");
            wait_for_press_of(Enter)
        }
        execute!(io::stdout(), RestorePosition, Clear(FromCursorDown))?;
    }
    Ok(())
}
