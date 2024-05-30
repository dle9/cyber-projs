use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use crate::app::{App, Screen, Editing};
use crate::util::input::InputMode;
use crate::screens::customize::Customize;

impl App {
    // updates the application's state based on user input
    pub fn handle_events(&mut self) -> std::io::Result<()> {
        match event::read()? {
            // user pressed key on keyboard
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
    
    // handle keyboard events 
    // each screen has specific keybindings
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.curr_screen {
            Screen::Intro => self.handle_intro_controls(key_event),
            Screen::Identify=> self.handle_identify_controls(key_event),
            Screen::Customize => self.handle_customize_controls(key_event),
            Screen::Welcome => self.handle_welcome_controls(key_event),
            Screen::Settings => self.handle_settings_controls(key_event),
            Screen::Exit => self.handle_exit_controls(key_event),
        }
    }

    fn handle_intro_controls(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => {
                self.prev_screen = self.curr_screen.clone();
                self.curr_screen = Screen::Exit;
            },
            KeyCode::Enter => {
                self.prev_screen = self.curr_screen.clone();
                self.curr_screen = Screen::Identify;
            },
            _ => {}
        }
    }

    fn handle_identify_controls(&mut self, key_event: KeyEvent) {
        match self.input.mode {

            InputMode::Normal => {
                match key_event.code {
                    KeyCode::Tab => {
                        self.player.name = self.input.insert_cursor();
                        self.editing = Some(Editing::Name);
                        self.toggle_input_mode();
                    },
                    KeyCode::Char('q') => {
                        self.prev_screen = self.curr_screen.clone();
                        self.curr_screen = Screen::Exit;
                    },
                    KeyCode::Enter => {
                        self.prev_screen = self.curr_screen.clone();
                        self.curr_screen = Screen::Customize;
                    },
                    _ => {}
                }
            },
            InputMode::Editing => {
                match key_event.code {
                    KeyCode::Tab => {
                        self.player.name = self.input.remove_cursor();
                        self.editing = None;
                        self.toggle_input_mode();
                    },
                    KeyCode::Char(c) => {
                        self.player.name = self.input.enter_char(c);
                    }
                    KeyCode::Left => {
                        self.player.name = self.input.move_cursor_left();
                    }
                    KeyCode::Right => {
                        self.player.name = self.input.move_cursor_right();
                    }
                    KeyCode::Backspace => {
                        self.player.name = self.input.delete_char();
                    }
                    _ => {}
                }
            },
        }
    }

    fn handle_customize_controls(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => {
                self.prev_screen = self.curr_screen.clone();
                self.curr_screen = Screen::Exit;
            },
            KeyCode::Enter => {
                self.prev_screen = self.curr_screen.clone();
                self.curr_screen = Screen::Customize;
            },
            KeyCode::Left => self.menus.customize.items.unselect(),
            KeyCode::Down => self.menus.customize.items.next(),
            KeyCode::Up => self.menus.customize.items.previous(),
            KeyCode::Right | KeyCode::Enter => {
                self.menus.customize.change_status();
            },
            _ => {}
        }
    }

    fn handle_welcome_controls(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => {
                self.prev_screen = self.curr_screen.clone();
                self.curr_screen = Screen::Exit;
            },
            KeyCode::Esc => {
                self.prev_screen = self.curr_screen.clone();
                self.curr_screen = Screen::Settings;
            },
            _ => {}
        }
    }

    fn handle_settings_controls(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => {
                self.curr_screen = self.prev_screen.clone();
            },
            _ => {}
        }
    }
    
    fn handle_exit_controls(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Esc => {
                self.curr_screen = self.prev_screen.clone();
            },
            _ => {}
        }
    }
}
