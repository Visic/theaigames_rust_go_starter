#[macro_use] extern crate enum_primitive;
extern crate rand;
extern crate num;
mod settings;
mod game_state;

use settings::*;
use game_state::*;
use std::io::{self, BufRead};
use rand::{StdRng, Rng};

fn main() {
    let mut settings = Settings::new();
    let mut game_state = GameState::new();
    let mut rng = StdRng::new().unwrap();
    let stdin = io::stdin();
    
    for line in stdin.lock().lines().map(|x| x.unwrap()) {    
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "settings" => settings.update(parts[1], parts[2]),
            "action" => make_move(parts[2], &settings, &game_state, &mut rng),
            "update" => game_state.update(parts[1], parts[2], parts[3]),
            _ => {}
        }
    }
}

fn make_move(_time: &str, settings: &Settings, _game_state: &GameState, rng: &mut StdRng) {
    //TODO:: Add game logic here
    
    let x = rng.next_u32() % (settings.field_width as u32);
    let y = rng.next_u32() % (settings.field_height as u32);
    println!("place_move {} {}", x, y);
    //println!("pass");
}