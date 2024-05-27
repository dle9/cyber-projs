use ratatui::{prelude::*, symbols::border, widgets::*};

pub fn render_settings_screen(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(" Settings ".bold())
        .borders(Borders::ALL)
        .border_set(border::THICK);
    let paragraph = Paragraph::new("Welcome to the Settings Screen!\nPress Esc to go back.")
        .block(block)
        .alignment(Alignment::Center);
    frame.render_widget(paragraph, area);
}