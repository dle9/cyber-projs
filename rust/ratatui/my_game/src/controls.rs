use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use crate::app::{App, Screen};

impl App {
    // updates the application's state based on user input
    pub fn handle_events(&mut self) -> std::io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    // handle keyboard events 
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.curr_screen {
            Screen::Intro => self.handle_intro_controls(key_event),
            Screen::Customize => self.handle_customize_controls(key_event),
            Screen::Welcome => self.handle_welcome_controls(key_event),
            Screen::Settings => self.handle_settings_controls(key_event),
        }
    }

    fn handle_intro_controls(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Esc => {
                self.prev_screen = self.curr_screen.clone();
                self.curr_screen = Screen::Settings;
            },
            KeyCode::Enter => {
                self.prev_screen = self.curr_screen.clone();
                self.curr_screen = Screen::Customize;
            }
            _ => {}
        }
    }

    fn handle_customize_controls(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Esc => {
                self.prev_screen = self.curr_screen.clone();
                self.curr_screen = Screen::Settings;
            },
            KeyCode::Enter => {
                self.prev_screen = self.curr_screen.clone();
                self.curr_screen = Screen::Welcome;
            }
            _ => {}
        }
    }

    fn handle_welcome_controls(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Esc => {
                self.prev_screen = self.curr_screen.clone();
                self.curr_screen = Screen::Settings;
            },
            _ => {}
        }
    }

    fn handle_settings_controls(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Esc => {
                self.curr_screen = self.prev_screen.clone();
            },
            _ => {}
        }
    }
}
