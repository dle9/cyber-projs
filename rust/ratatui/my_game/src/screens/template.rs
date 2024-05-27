use ratatui::{prelude::*, symbols::border, widgets::*};

pub fn render_selection_screen(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(" Template Screen ".bold())
        .borders(Borders::ALL)
        .border_set(border::THICK);
    let paragraph = Paragraph::new("This is the template screen.")
        .block(block)
        .alignment(Alignment::Center);
    frame.render_widget(paragraph, area);
}