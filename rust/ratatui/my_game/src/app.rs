use ratatui::{prelude::*};

use crate::player::Player;
use crate::screens::{intro::*, selection::*, welcome::*, settings::*};
// use crate::screens::intro::render_intro_screen;
// use crate::screens::selection::render_selection_screen;
// use crate::screens::welcome::render_welcome_screen;

// type alias of variable to pass to App::run()
type Tui = ratatui::Terminal<ratatui::prelude::CrosstermBackend<std::io::Stdout>>;

// all windows of the app
#[derive(Clone)]
pub enum Screen {
    Intro, // start, settings, exit
    Selection, // Intro::Start -> name, class, skillpoints
    Welcome, // Selection -> welcome message with Selections
    Settings, // * -> . || . -> prev
}

pub struct App {
    player: Player,
    exit: bool,
    pub prev_screen: Screen,
    pub curr_screen: Screen,
}

// app main loop
// each iteration draws one frame
impl App {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            exit: false,
            curr_screen: Screen::Intro,
            prev_screen: Screen::Intro,
        }
    }
    
    // runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut Tui) -> std::io::Result<()> {
        while !self.exit {
            // |closure|: anonymous funcs you can pass to another func  
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?; // controls.rs
        }

        Ok(())
    }

    // render app screens as widgets
    fn render_frame(&self, frame: &mut Frame) {
        match self.curr_screen {
            Screen::Intro => render_intro_screen(frame, frame.size()),
            Screen::Selection => render_selection_screen(frame, frame.size()),
            Screen::Welcome => render_welcome_screen(frame, frame.size()),
            Screen::Settings => render_settings_screen(frame, frame.size()),
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
}