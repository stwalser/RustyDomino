mod client;
mod game;
mod player;
mod protocol;
mod queue;
mod stone;

use crate::client::Client;
use crate::game::Game;
use crate::player::Player;
use crate::protocol::{ClientAnswer, GameCommand};
use crossterm::cursor::{MoveTo, RestorePosition, SavePosition};
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
        let mut n_players_string = String::new();
        io::stdin().read_line(&mut n_players_string)?;
        let Ok(n) = n_players_string.trim().parse::<usize>() else {
            execute!(io::stdout(), RestorePosition, Clear(FromCursorDown))?;
            println!("Please enter a number! ");
            continue;
        };
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
                Client::new(cmd_rx, ans_tx, vec![], name.to_string()),
                Player::new(name.to_string(), cmd_tx, ans_rx),
            )
        })
        .unzip();

    for mut client in clients {
        std::thread::spawn(move || {
            client.run();
        });
    }

    let mut game = Game::new(players);

    game.init_players_stones();

    game.determine_start_player();

    loop {
        game.handle_move();

        execute!(io::stdout(), RestorePosition, Clear(FromCursorDown))?;
        if game.current_player_won() {
            game.add_bonus_points();
            game.print_points();
            break;
        } else {
            game.next_player();
        }
    }

    Ok(())
}
