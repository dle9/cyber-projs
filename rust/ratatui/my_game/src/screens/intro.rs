use ratatui::{prelude::*, symbols::border, widgets::*};

pub fn render_intro_screen(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(" Intro Screen ".bold())
        .borders(Borders::ALL)
        .border_set(border::THICK);
    let paragraph = Paragraph::new("Welcome to the Intro Screen!\nPress Enter to continue.")
        .block(block)
        .alignment(Alignment::Center);
    frame.render_widget(paragraph, area);
}