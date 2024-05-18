// init valid options for
// PLAY command
#[derive(Debug)]
pub struct GameOptions {
    pub game_options: Vec<&'static str>, 
}

// init valid options for
// SHOW and CHANGE commands
#[derive(Debug)]
pub struct PlayerOptions {
    pub show_options: Vec<&'static str>, 
    pub change_options: Vec<&'static str>, 
}