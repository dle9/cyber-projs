use ratatui::{prelude::*};

use crate::player::Player;
use crate::util::input::{Input, InputMode};
use crate::screens::customize::Customize;

// type alias of variable to pass to App::run()
type Tui = ratatui::Terminal<ratatui::prelude::CrosstermBackend<std::io::Stdout>>;

// all windows of the app
#[derive(Clone)]
pub enum Screen {
    Intro, 
    Identify, // Intro -> name
    Customize, // Identify -> class, skillpoints
    Welcome, // Customize -> welcome message
    Settings,
    Exit,
}

pub struct Menus {
    pub customize: Customize,
}

#[derive(Clone)]
pub enum Editing {
    Name,
    Class,
    Skills,
}

pub struct App {
    pub player: Player,
    pub prev_screen: Screen,
    pub curr_screen: Screen,
    pub input: Input,
    pub editing: Option<Editing>,
    pub menus: Menus,
    exit: bool,
}

// app main loop
// each iteration draws one frame
impl App {
    pub fn new() -> Self {
        Self {  
            player: Player::new(),
            curr_screen: Screen::Intro,
            prev_screen: Screen::Intro,
            input: Input::new(),
            editing: None,
            menus: Menus {
                customize: Customize::new(),
            },
            exit: false,
        }
    }
    
    // runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut Tui) -> std::io::Result<()> {
        while !self.exit {
            // |closure|: anonymous funcs you can pass to another func  
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?; // src/controls.rs
        }

        Ok(())
    }

    // render app screens as widgets
    fn render_frame(&mut self, frame: &mut Frame) {
        match self.curr_screen { 
            // src/screens/render_*_screen.rs
            Screen::Intro => self.render_intro_screen(frame, frame.size()),
            Screen::Identify => self.render_identify_screen(frame, frame.size()),
            Screen::Customize => self.render_customize_screen(frame, frame.size()),
            Screen::Welcome => self.render_welcome_screen(frame, frame.size()),
            Screen::Settings => self.render_settings_screen(frame, frame.size()),
            Screen::Exit => self.render_exit_screen(frame, frame.size()),
        }
    }

    pub fn toggle_input_mode(&mut self) {
        if matches!(self.input.mode, InputMode::Normal) {
            self.input.mode = InputMode::Editing
        } else {
            self.input.mode = InputMode::Normal
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
}