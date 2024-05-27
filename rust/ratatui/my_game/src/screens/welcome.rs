use ratatui::{prelude::*, symbols::border, widgets::*};

pub fn render_welcome_screen(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(" Welcome Screen ".bold())
        .borders(Borders::ALL)
        .border_set(border::THICK);
    let paragraph = Paragraph::new("Welcome to the Welcome Screen!\nPress 'i' to go back to the Intro Screen.")
        .block(block)
        .alignment(Alignment::Center);
    frame.render_widget(paragraph, area);
}