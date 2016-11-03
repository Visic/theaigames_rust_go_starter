use rand::{StdRng, Rng};
use game_state::*;
use settings::*;

pub struct GameLogic {
    game_state: GameState,
    settings: Settings,
    rng: StdRng
}

impl GameLogic {
    pub fn new() -> GameLogic {
        GameLogic{ game_state: GameState::new(), settings: Settings::new(), rng: StdRng::new().unwrap() }
    }
    
    pub fn process_input(&mut self, line: &str) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "settings" => self.settings.update(parts[1], parts[2]),
            "action" => self.make_move(parts[2]),
            "update" => self.game_state.update(parts[1], parts[2], parts[3]),
            _ => {}
        }
    }
    
    fn make_move(&mut self, _time: &str) {
        //TODO:: Change out the random moves for something cooler
        let xpos = self.rng.gen::<u32>() % self.settings.field_width;
        let ypos = self.rng.gen::<u32>() % self.settings.field_height;
        
        if self.validate_move(xpos, ypos) {
            println!("place_move {} {}", xpos, ypos);
        } else {
            println!("pass");
        }
    }

    fn validate_move(&self, xpos: u32, ypos: u32) -> bool {
        //Note:: This will also handle the KO rule
        let is_empty = self.get_point(xpos, ypos) == Point::Empty;
        if !is_empty { return false; }
        
        //one side must be empty (e.g. a new peice at this position must have at least one liberty)
        let is_suicide = (xpos == 0                              || self.get_point(xpos - 1, ypos)     != Point::Empty) &&
                         (xpos + 1 >= self.settings.field_width  || self.get_point(xpos + 1, ypos)     != Point::Empty) &&
                         (ypos == 0                              || self.get_point(xpos,     ypos - 1) != Point::Empty) &&
                         (ypos + 1 >= self.settings.field_height || self.get_point(xpos,     ypos + 1) != Point::Empty);
        !is_suicide
    }

    fn get_point(&self, xpos: u32, ypos: u32) -> Point {
        let index = GameLogic::coods_to_index(xpos, ypos, self.settings.field_width) as usize;
        self.game_state.field[index]
    }

    fn coods_to_index(xpos: u32, ypos: u32, width: u32) -> u32 {
        width * ypos + xpos
    }
}