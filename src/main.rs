mod client;
mod common;
mod kernel;
mod utils;

use crate::client::Client;
use crate::common::protocol::{ClientAnswer, GameCommand};
use crate::kernel::communicator::local::Local;
use crate::kernel::game::{Game, GameMode, System};
use crate::kernel::player::Player;
use crate::utils::{get_int_from_input, wait_for_press_of};
use crossterm::cursor::{MoveTo, RestorePosition, SavePosition};
use crossterm::event::KeyCode::Char;
use crossterm::execute;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType::FromCursorDown;
use std::io;
use std::sync::mpsc::sync_channel;

fn main() -> io::Result<()> {
    execute!(io::stdout(), MoveTo(0, 0), Clear(FromCursorDown))?;
    println!("---RustyDomino---");
    execute!(io::stdout(), SavePosition)?;
    println!("Welcome to RustyDomino!");
    let n_players = loop {
        println!("How many people will play (2-4)?");
        let n = get_int_from_input();
        if !(2..=4).contains(&n) {
            execute!(io::stdout(), RestorePosition, Clear(FromCursorDown))?;
            println!("{} players are not supported! ", n);
            continue;
        }
        break n;
    };

    let (clients, players): (Vec<Client>, Vec<Player>) = (0..n_players)
        .map(|id| {
            execute!(io::stdout(), RestorePosition, Clear(FromCursorDown)).unwrap();
            println!("Please enter the name of person {}:", id + 1);
            let mut name_string = String::new();
            io::stdin().read_line(&mut name_string).unwrap();

            let (cmd_tx, cmd_rx) = sync_channel::<GameCommand>(0);
            let (ans_tx, ans_rx) = sync_channel::<ClientAnswer>(0);
            let name = name_string.trim();
            (
                Client::new(cmd_rx, ans_tx),
                Player::new(name.to_string(), Box::new(Local::new(cmd_tx, ans_rx))),
            )
        })
        .unzip();

    execute!(io::stdout(), RestorePosition, Clear(FromCursorDown))?;
    println!("Which system do you want to play?");
    println!("1) Double 6");
    println!("2) Double 9");
    let system = match wait_for_press_of(vec![Char('1'), Char('2')])? {
        Char('1') => System::Double6,
        Char('2') => System::Double9,
        _ => unreachable!(),
    };

    execute!(io::stdout(), RestorePosition, Clear(FromCursorDown))?;
    println!("Which mode do you want to play?");
    println!("1) Fixed number of rounds");
    println!("2) Fixed number of points");
    let mode = match wait_for_press_of(vec![Char('1'), Char('2')])? {
        Char('1') => {
            println!("How many rounds do you want to play?");
            GameMode::Rounds(get_int_from_input())
        }
        Char('2') => {
            println!("Until many points do you want to play?");
            GameMode::Points(get_int_from_input())
        }
        _ => unreachable!(),
    };

    for mut client in clients {
        std::thread::spawn(move || {
            client.run();
        });
    }

    Game::new(players, mode, system).run();

    Ok(())
}
