mod game;
mod player;
mod queue;
mod stone;

use crate::game::{Game, GameError};
use crate::player::Player;
use crate::queue::QueueError;
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
            println!("{} players are not supported!!!", n)
        } else {
            println!("Please enter a number!!!")
        }
    };

    let players = (0..n_players)
        .map(|id| {
            println!("Bitte gib den Namen von Person {} ein", id);
            let mut name_string = String::new();
            io::stdin().read_line(&mut name_string).unwrap();
            Player::new(name_string, id)
        })
        .collect::<Vec<Player>>();

    let mut game = Game::new(players);
    game.init_players_stones();

    game.determine_start_player();

    loop {
        print_current_round(&mut game);
        if let Err(_) = game.draw_for_current_player() {}
        game.next_player();
    }
}

fn print_current_round(game: &mut Game) {
    game.print_queue();

    let player = game.get_current_player();

    player.print_round_greeting();
    player.print_stones();

    loop {
        println!("Which stone do you want to play?");
        let mut chosen_stone_string = String::new();
        io::stdin().read_line(&mut chosen_stone_string).unwrap();
        if let Ok(n) = chosen_stone_string.trim().parse::<usize>() {
            match game.play_stone(n) {
                Ok(()) => {
                    break;
                }
                Err(GameError::Queue(QueueError::DoubleNecessary)) => {
                    println!("Start with a double!")
                }
                Err(e) => println!("{e}"),
            }
        } else {
            println!("Enter a number to choose a stone!!!")
        }
    }
}
