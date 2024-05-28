use ratatui::{prelude::*, layout::*, symbols::border, widgets::{*, block::*}};

use crate::app::App;

impl App {
    pub fn render_intro_screen(&self, frame: &mut Frame, area: Rect) {
        // let title = Title::from(" Walls of Descent ".bold());
        // let controls = Title::from(Line::from(vec![
        //     " Settings ".into(),
        //     "<Esc>".blue().bold(),
        //     " Continue ".into(),
        //     "<Enter> ".blue().bold(),
        // ]));
        
        // let block = Block::default()
        //     .title(title.alignment(Alignment::Left))
        //     .title(
        //         controls
        //             .alignment(Alignment::Left)
        //             .position(Position::Bottom),
        //     )
        //     .borders(Borders::ALL)
        //     .border_set(border::THICK);
        
        // let paragraph = Paragraph::new("")
        //     .centered()
        //     .block(block);
        // frame.render_widget(paragraph, area);

        

        // partition the frame
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(57),
                Constraint::Min(12),
                Constraint::Percentage(43),
                // Constraint::Length(3),
            ]).split(area);
                
        // create the intro message
        // let intro_msg = vec![
        //     Span::from("Press "),
        //     Span::styled("Enter ", Style::default().add_modifier(Modifier::ITALIC)),
        //     Span::from("to begin"),
        // ];
        // let title = Span::styled(" Walls of Descent ", Style::default().fg(Color::Green)); 
        let intro_block = Block::default()
            // .title(title.alignment(Alignment::Center))
            .borders(Borders::ALL)
            .style(Style::default());

        let intro_msg= vec![
            Line::from(vec![
                Span::raw("First"),
                Span::styled("line", Style::new().green().italic()),
                ".".into(),
            ]),
            Line::from("Second line".red()),
            "Third line".into(),
        ];
        let intro = Paragraph::new(intro_msg).block(intro_block).alignment(Alignment::Center);

        // render the title into the second chunk
        frame.render_widget(intro, chunks[1]);
    }
}