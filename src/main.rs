#[macro_use] extern crate enum_primitive;
extern crate rand;
extern crate num;

mod settings;
mod game_state;
mod game_logic;

use game_logic::*;
use std::io::{self, BufRead};

fn main() {
    let mut game_logic = GameLogic::new();
    let stdin = io::stdin();
    
    for line in stdin.lock().lines().map(|x| x.unwrap()) {    
        game_logic.process_input(&line);
    }
}