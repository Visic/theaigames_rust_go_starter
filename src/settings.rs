pub struct Settings {
    pub timebank: i32,
    pub time_per_move: u32,
    pub player_names: Vec<String>,
    pub your_bot: String,
    pub your_botid: i32,
    pub field_width: u32,
    pub field_height: u32,
    pub max_rounds: u32
}

impl Settings {
    pub fn new() -> Settings {
        Settings{ timebank: 0, time_per_move: 0, player_names: vec![], your_bot: String::from(""), your_botid: 0, field_width: 0, field_height: 0, max_rounds: 0 }
    }
    
    pub fn update(&mut self, setting: &str, value_str: &str) {
        match setting.as_ref() {
            "timebank"      => self.timebank = value_str.parse::<i32>().unwrap(),
            "time_per_move" => self.time_per_move = value_str.parse::<u32>().unwrap(),
            "player_names"  => self.player_names = value_str.split(",").map(|x| String::from(x)).collect(),
            "your_bot"      => self.your_bot = String::from(value_str),
            "your_botid"    => self.your_botid = value_str.parse::<i32>().unwrap(),
            "field_width"   => self.field_width = value_str.parse::<u32>().unwrap(),
            "field_height"  => self.field_height = value_str.parse::<u32>().unwrap(),
            "max_rounds"    => self.max_rounds = value_str.parse::<u32>().unwrap(),
            _ => {}
        }
    }
}