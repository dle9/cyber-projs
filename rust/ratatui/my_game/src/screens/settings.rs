use ratatui::{prelude::*, symbols::border, widgets::{*, block::*}};

use crate::app::App;

impl App {
    pub fn render_settings_screen(&self, frame: &mut Frame, area: Rect) {
        let title = Title::from(" Settings ".bold());
        let controls = Title::from(Line::from(vec![
            " Return ".into(),
            "<Esc> ".blue().bold(),
        ]));
        
        let block = Block::default()
            .title(title.alignment(Alignment::Left))
            .title(
                controls
                    .alignment(Alignment::Left)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);
        
        let paragraph = Paragraph::new("")
            .centered()
            .block(block);
        
        frame.render_widget(paragraph, area);
    }
}