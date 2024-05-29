use ratatui::{prelude::*, widgets::{*, block::{*, Position}}};

use crate::app::App;
use crate::util::input::InputMode;
use crate::util::colors;

impl App {
    pub fn render_customize_screen(&self, frame: &mut Frame, area: Rect) {
        match self.input.mode {
            InputMode::Normal => {
                self.render_normal(frame, area);
            },
            InputMode::Editing => {
                self.render_editing(frame, area);
            }
        }
    }
    
    fn render_normal(&self, frame: &mut Frame, area: Rect) {
        // partition the frame
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(60),
                Constraint::Min(9),
                Constraint::Percentage(40),
            ]).split(area);
        
        // top and bottom of the block
        let title = Line::from(Span::styled(
            "Viewing your character",
            Style::default().fg(colors::MAIN).add_modifier(Modifier::BOLD),
        ));
        let controls = Title::from(Line::from(vec![
            Span::from("Quit"),
            Span::styled("<q> ", Style::default().fg(colors::HIGHLIGHT).add_modifier(Modifier::BOLD)),
            Span::from("Edit"),
            Span::styled("<Tab> ", Style::default().fg(colors::HIGHLIGHT).add_modifier(Modifier::BOLD)),
            Span::from("Continue"),
            Span::styled("<Enter>", Style::default().fg(colors::HIGHLIGHT).add_modifier(Modifier::BOLD)),
        ]));

        // create the block (container)
        let block = Block::default()
            .title(title)
            .title(controls.position(Position::Bottom))
            .borders(Borders::ALL)
            .style(Style::default());

        // text inside the block
        let msg = vec![
            // Line::from(vec![Span::from("")]),
            // Line::from(vec![Span::from("")]),
            // Line::from(vec![Span::from("")]),
            Line::from(vec![Span::styled(
                format!("Username: {}", &self.player.name),
                Style::default().fg(colors::HIGHLIGHT),
            )]),
        ];

        // render the intro msg into the second chunk
        frame.render_widget(Paragraph::new(msg).block(block).alignment(Alignment::Center), chunks[1]);
    }

    fn render_editing(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(60),
                Constraint::Min(9),
                Constraint::Percentage(40),
            ]).split(area);
        
        // top and bottom of the block
        let title = Line::from(Span::styled(
            "Editing",
            Style::default().fg(colors::MAIN).add_modifier(Modifier::BOLD),
        ));
        let controls = Title::from(Line::from(vec![
            Span::from("Normal"),
            Span::styled("<Tab> ", Style::default().fg(colors::HIGHLIGHT).add_modifier(Modifier::BOLD)),
            Span::from("Switch value"),
            Span::styled("<Enter>", Style::default().fg(colors::HIGHLIGHT).add_modifier(Modifier::BOLD)),
        ]));

        // create the block (container)
        let block = Block::default()
            .title(title)
            .title(controls.position(Position::Bottom))
            .borders(Borders::ALL)
            .style(Style::default());

        // text inside the block
        let msg = vec![
            // Line::from(vec![Span::from("")]),
            // Line::from(vec![Span::from("")]),
            // Line::from(vec![Span::from("")]),
            Line::from(vec![Span::styled(
                format!("Username: {}", &self.player.name),
                Style::default().fg(colors::HIGHLIGHT),
            )]),
        ];

        // render the intro msg into the second chunk
        frame.render_widget(Paragraph::new(msg).block(block).alignment(Alignment::Center), chunks[1]);
    }
}