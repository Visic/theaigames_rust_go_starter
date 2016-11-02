#[macro_use] extern crate enum_primitive;
extern crate num;
mod settings;
mod game_state;

use settings::*;
use game_state::*;
use std::io::{self, BufRead};

fn main() {
    let mut settings = Settings::new();
    let mut game_state = GameState::new();
    let stdin = io::stdin();
    
    for line in stdin.lock().lines().map(|x| x.unwrap()) {    
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "settings" => settings.update(parts[1], parts[2]),
            "action" => make_move(parts[2], &settings, &game_state),
            "update" => game_state.update(parts[1], parts[2], parts[3]),
            _ => {}
        }
    }
}

fn make_move(_time: &str, _settings: &Settings, _game_state: &GameState) {
    //TODO:: Add game logic here
    
    // println!("place_move {} {}", 0, 0);
    println!("pass");
}