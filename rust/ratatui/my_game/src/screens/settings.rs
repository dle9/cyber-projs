use ratatui::{prelude::*, widgets::{*, block::{*, Position}}};

use crate::app::App;
use crate::util::colors;

impl App {
    pub fn render_settings_screen(&self, frame: &mut Frame, area: Rect) {
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
            "Settings",
            Style::default().fg(colors::MAIN).add_modifier(Modifier::BOLD),
        ));
        let controls = Title::from(Line::from(vec![
            Span::from("Return"),
            Span::styled("<Esc>", Style::default().fg(colors::HIGHLIGHT).add_modifier(Modifier::BOLD)),
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
            Line::from(vec![
                Span::styled("Settings", Style::default().add_modifier(Modifier::ITALIC)),
            ]),
        ];

        // render the intro msg into the second chunk
        frame.render_widget(Paragraph::new(msg).block(block).alignment(Alignment::Center), chunks[1]);
    }
}