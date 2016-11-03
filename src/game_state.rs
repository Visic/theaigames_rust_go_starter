use num::FromPrimitive;

enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq)]
    pub enum Point {
        Illegal = -1,
        Empty,
        Player1,
        Player2,
    }
}

pub struct GameState {
    pub round: i32,
    pub moves_done: i32,
    pub player1_points: f64,
    pub player2_points: f64,
    pub field: Vec<Point>
}

impl GameState {
    pub fn new() -> GameState {
        GameState{ round: 0, moves_done: 0, player1_points: 0.0, player2_points: 0.0, field: vec![] }
    }
    
    pub fn update(&mut self, category: &str, setting: &str, value_str: &str) {
        match setting.as_ref() {
            "round"                              => self.round = value_str.parse::<i32>().unwrap(),
            "move"                               => self.moves_done = value_str.parse::<i32>().unwrap(),
            "points"    if category == "player1" => self.player1_points = value_str.parse::<f64>().unwrap(),
            "points"    if category == "player2" => self.player2_points = value_str.parse::<f64>().unwrap(),
            "last_move" if category == "player1" => { /*example value_str: place_move 15 1*/ },
            "last_move" if category == "player2" => { /*example value_str: place_move 15 1*/ },
            "field"                              => self.field = value_str.split(",").map(|x| Point::from_i32(x.parse::<i32>().unwrap()).unwrap()).collect(),
            _ => {}
        }
    }
}