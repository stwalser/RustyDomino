mod game;
mod player;
mod queue;
mod stone;

use crate::game::Game;
use crate::player::Player;
use crate::queue::Playable;
use crossterm::cursor::{MoveTo, RestorePosition, SavePosition};
use crossterm::event::KeyCode::{Char, Enter};
use crossterm::event::{Event, KeyCode, read};
use crossterm::execute;
use crossterm::terminal::ClearType::FromCursorDown;
use crossterm::terminal::{Clear, disable_raw_mode, enable_raw_mode};
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
        let Ok(n) = n_players_string.trim().parse::<usize>() else {
            execute!(io::stdout(), RestorePosition, Clear(FromCursorDown))?;
            println!("Please enter a number! ");
            continue;
        };
        if n < 2 || n > 4 {
            execute!(io::stdout(), RestorePosition, Clear(FromCursorDown))?;
            println!("{} players are not supported! ", n);
            continue;
        }
        break n;
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

    loop {
        game.init_players_stones();

        game.determine_start_player();

        execute!(io::stdout(), RestorePosition, Clear(FromCursorDown))?;
        loop {
            game.print_round_greeting();
            wait_for_press_of(vec![Enter])?;
            execute!(io::stdout(), RestorePosition, Clear(FromCursorDown))?;
            print_current_round(&mut game)?;
            execute!(io::stdout(), RestorePosition, Clear(FromCursorDown))?;
            if game.current_player_won() {
                game.add_bonus_points();
                game.print_points();
                break;
            } else {
                game.next_player();
            }
        }

        println!("Do you want to play another round? (y)es/(n)o");
        let pressed = wait_for_press_of(vec![Char('y'), Char('n')])?;
        if pressed == Char('y') {
            continue;
        } else if pressed == Char('n') {
            break;
        }
    }

    Ok(())
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

fn print_current_round(game: &mut Game) -> io::Result<()> {
    let mut last_round_feedback = "".to_string();

    loop {
        execute!(io::stdout(), RestorePosition, Clear(FromCursorDown))?;
        game.print_board();
        game.print_current_player_stones();

        let mut input_string = String::new();

        println!("{}", last_round_feedback);
        if game.current_player_can_make_a_move() {
            println!("Which stone do you want to play?");
            io::stdin().read_line(&mut input_string)?;
            let input = input_string.trim().to_string();

            let Ok(n) = input.parse::<usize>() else {
                last_round_feedback = "Enter a number to choose a stone! ".to_string();
                continue;
            };

            let mut playable = match game.check_stone(n - 1) {
                Ok(p) => p,
                Err(e) => {
                    last_round_feedback = e.to_string();
                    continue;
                }
            };

            if playable == Playable::Both {
                println!("Do you want to play the stone on the (l)eft or the (r)ight side?");
                let pressed = wait_for_press_of(vec![Char('l'), Char('r')])?;
                if pressed == Char('l') {
                    playable = Playable::Left;
                } else if pressed == Char('r') {
                    playable = Playable::Right;
                }
            }

            match game.play_stone(n - 1, playable) {
                Ok(_) => {
                    game.check_for_points();
                    break;
                }
                Err(e) => {
                    last_round_feedback = e.to_string();
                }
            }
        } else if game.talon_not_empty() {
            println!("You cannot make a move. Press 'd' for drawing a stone from the talon.");
            wait_for_press_of(vec![Char('d')])?;
            game.draw_for_current_player()
        } else {
            println!("You cannot make a move and the talon is empty. Press 'enter' to confirm");
            wait_for_press_of(vec![Enter])?;
        }
    }
    Ok(())
}
