use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{prelude::*, symbols::border, widgets::{*, block::*}};

use crate::player::Player;

// type alias of variable to pass to App::run()
type Tui = ratatui::Terminal<ratatui::prelude::CrosstermBackend<std::io::Stdout>>;

// the current window the user is on
pub enum CurrentScreen {
    Intro, // start, settings, exit
    Selection, // Intro::Start -> name, class, skillpoints
    Welcome, // Selection -> welcome message with Selections
    Exiting, // Intro::Exit
}

pub struct App {
    player: Player,
    exit: bool,
}

// app main loop
// each iteration draws one frame
impl App {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            exit: false,
        }
    }
    
    // runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut Tui) -> std::io::Result<()> {
        while !self.exit {
            // |closure|: anonymous funcs you can pass to another func  
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    // render app as a widget
    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    // updates the application's state based on user input
    fn handle_events(&mut self) -> std::io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    // handle keyboard events
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            // KeyCode::Left => self.decrement_counter(),
            // KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

// implement widget trait for app
// https://ratatui.rs/concepts/widgets/
// organizes code for app rendering
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Counter App Tutorial ".bold());
        let instructions = Title::from(Line::from(vec![
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Left)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        Paragraph::new("What's your name?")
            .centered()
            .block(block)
            .render(area, buf);
    }
}