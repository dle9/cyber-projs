use ratatui::{prelude::*, widgets::{*, block::{*, Position}}};

use crate::app::{App, Editing};
use crate::util::input::InputMode;
use crate::util::colors;

impl App {
    pub fn render_identify_screen(&mut self, frame: &mut Frame, area: Rect) {
        match self.input.mode {
            InputMode::Normal => {
                self.editing = None;
                self.identity_normal(frame, area);
            },
            InputMode::Editing => {
                self.editing = Some(Editing::Name);
                self.identity_editing(frame, area);
            }
        }
    }

    fn identity_normal(&self, frame: &mut Frame, area: Rect) {
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
            "What's your name?",
            Style::default().fg(colors::MAIN).add_modifier(Modifier::BOLD),
        ));
        let controls = Title::from(Line::from(vec![
            Span::from("Quit"),
            Span::styled("<q> ", Style::default().fg(colors::HIGHLIGHT).add_modifier(Modifier::BOLD)),
            Span::from("Edit"),
            Span::styled("<Tab> ", Style::default().fg(colors::HIGHLIGHT).add_modifier(Modifier::BOLD)),
            Span::from("Confirm"),
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
            Line::from(vec![Span::from("")]),
            Line::from(vec![Span::from("")]),
            Line::from(vec![Span::from("")]),
            Line::from(vec![Span::styled(
                format!("Name: {}", &self.player.name),
                Style::default().fg(colors::HIGHLIGHT),
            )]),
        ];

        // render the intro msg into the second chunk
        frame.render_widget(Paragraph::new(msg).block(block).alignment(Alignment::Center), chunks[1]);
    }

    fn identity_editing(&self, frame: &mut Frame, area: Rect) {
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
            Span::from("Return"),
            Span::styled("<Tab>", Style::default().fg(colors::HIGHLIGHT).add_modifier(Modifier::BOLD)),
        ]));

        // create the block (container)
        let block = Block::default()
            .title(title)
            .title(controls.position(Position::Bottom))
            .borders(Borders::ALL)
            .style(Style::default());

        // text inside the block
        let msg = vec![
            Line::from(vec![Span::from("")]),
            Line::from(vec![Span::from("")]),
            Line::from(vec![Span::from("")]),
            Line::from(vec![Span::styled(
                format!("Name: {}", &self.player.name),
                Style::default().fg(colors::HIGHLIGHT),
            )]),
        ];

        // render the intro msg into the second chunk
        frame.render_widget(Paragraph::new(msg).block(block).alignment(Alignment::Center), chunks[1]);
    }
}