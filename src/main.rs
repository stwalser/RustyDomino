mod clients;
mod common;
mod kernel;
mod utils;

use crate::clients::client::Client;
use crate::clients::local::LocalReceiver;
use crate::common::protocol::{ClientAnswer, GameCommand};
use crate::kernel::communicator::local::Local;
use crate::kernel::communicator::remote::Remote;
use crate::kernel::game::{Game, GameMode, System};
use crate::kernel::player::Player;
use crate::utils::{get_int_from_input, wait_for_press_of};
use crossterm::cursor::{MoveTo, RestorePosition, SavePosition};
use crossterm::event::KeyCode::{Char, Enter};
use crossterm::execute;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType::FromCursorDown;
use futures_util::{SinkExt, StreamExt};
use std::io;
use std::io::Error;
use std::sync::mpsc::sync_channel;
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::{Message, Utf8Bytes};
use tokio_tungstenite::{accept_async, connect_async};
use crate::clients::remote::RemoteReceiver;

#[tokio::main]
async fn main() -> io::Result<()> {
    execute!(io::stdout(), MoveTo(0, 0), Clear(FromCursorDown))?;
    println!("---RustyDomino---");
    execute!(io::stdout(), SavePosition)?;
    println!("Welcome to RustyDomino!");

    println!("Which mode do you want to play?");
    println!("1) Local");
    println!("2) Remote");
    match wait_for_press_of(vec![Char('1'), Char('2')], true)? {
        Char('1') => local_game().await?,
        Char('2') => remote_game().await?,
        _ => unreachable!(),
    }
    Ok(())
}

fn ask_for_game_mode() -> Result<GameMode, Error> {
    execute!(io::stdout(), RestorePosition, Clear(FromCursorDown))?;
    println!("Which mode do you want to play?");
    println!("1) Fixed number of rounds");
    println!("2) Fixed number of points");
    let mode = match wait_for_press_of(vec![Char('1'), Char('2')], true)? {
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
    Ok(mode)
}

fn ask_for_system() -> Result<System, Error> {
    execute!(io::stdout(), RestorePosition, Clear(FromCursorDown))?;
    println!("Which system do you want to play?");
    println!("1) Double 6");
    println!("2) Double 9");
    let system = match wait_for_press_of(vec![Char('1'), Char('2')], true)? {
        Char('1') => System::Double6,
        Char('2') => System::Double9,
        _ => unreachable!(),
    };
    Ok(system)
}

async fn local_game() -> io::Result<()> {
    let system = ask_for_system()?;
    let game_mode = ask_for_game_mode()?;

    execute!(io::stdout(), RestorePosition, Clear(FromCursorDown))?;
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
            let name = name_string.trim();
            let (cmd_tx, cmd_rx) = sync_channel::<GameCommand>(0);
            let (ans_tx, ans_rx) = sync_channel::<ClientAnswer>(0);
            (
                Client::new(Box::new(LocalReceiver::new(cmd_rx, ans_tx))),
                Player::new(name.to_string(), Box::new(Local::new(cmd_tx, ans_rx))),
            )
        })
        .unzip();

    let mut game = Game::new(players, game_mode, system);
    let mut handles = Vec::new();
    for mut client in clients {
        let handle = tokio::task::spawn(async move {
            client.run().await;
        });
        handles.push(handle);
    }

    game.run().await;

    for handle in handles {
        handle.await.ok();
    }

    Ok(())
}

async fn remote_game() -> io::Result<()> {
    execute!(io::stdout(), RestorePosition, Clear(FromCursorDown))?;
    println!("Do you want to host a game or join an existing game?");
    println!("1) Host");
    println!("2) Join");
    match wait_for_press_of(vec![Char('1'), Char('2')], true)? {
        Char('1') => host_remote_game().await?,
        Char('2') => join_remote_game().await?,
        _ => unreachable!(),
    }
    Ok(())
}

async fn host_remote_game() -> io::Result<()> {
    let system = ask_for_system()?;
    let game_mode = ask_for_game_mode()?;

    let listener = TcpListener::bind("127.0.0.1:45678").await?;

    let mut players: Vec<Player> = vec![];
    let (start_tx, mut start_rx) = mpsc::channel::<()>(1);

    loop {
        execute!(io::stdout(), RestorePosition, Clear(FromCursorDown))?;
        println!("Currently connected players");
        for player in &players {
            println!("{}", player.name())
        }
        if players.len() >= 2 {
            println!("Press enter to start the game");
            // Spawn a task to wait for Enter key and signal through the channel
            let tx = start_tx.clone();
            tokio::task::spawn_blocking(move || {
                let _ = wait_for_press_of(vec![Enter], false);
                let _ = tx.blocking_send(());
            });
        } else {
            println!("At least two players necessary to start the game")
        }

        tokio::select! {
            res = listener.accept() => {
                let (stream, _) = res?;
                let Ok(mut ws) = accept_async(stream).await else { continue };
                let name = match ws.next().await.unwrap() {
                    Ok(Message::Text(text)) => {
                        text.to_string()
                    }
                    Ok(_) => continue,
                    Err(_) => continue,
                };
                ws.send(Message::Text(Utf8Bytes::from("received"))).await.expect("The client didn't receive the confirmation");
                players.push(Player::new(name, Box::new(Remote::new(ws))));
            }
            _ = start_rx.recv() => {
                break;
            }
        }
    }

    drop(listener);

    execute!(io::stdout(), RestorePosition, Clear(FromCursorDown))?;
    println!("Game started: Ding Ding Ding");

    Game::new(players, game_mode, system).run().await;

    Ok(())
}

async fn join_remote_game() -> io::Result<()> {
    execute!(io::stdout(), RestorePosition, Clear(FromCursorDown)).unwrap();
    println!("Please enter your name");
    let mut name_string = String::new();
    io::stdin().read_line(&mut name_string)?;
    let name = name_string.trim();

    execute!(io::stdout(), RestorePosition, Clear(FromCursorDown))?;
    println!("Please enter the IP address of the host");
    let mut ip_address = String::new();
    io::stdin().read_line(&mut ip_address)?;
    let ip_address = ip_address.trim();

    let (mut ws, _response) = connect_async("ws://".to_owned() + ip_address + ":45678")
        .await
        .expect("failed to connect");

    ws.send(Message::text(name)).await.unwrap();
    if let Some(Ok(Message::Text(text))) = ws.next().await
        && text == "received"
    {
    } else {
        panic!("The connection couldn't be established")
    }

    let mut client = Client::new(Box::new(RemoteReceiver::new(ws)));
    client.run().await;

    Ok(())
}
