use ratatui::{prelude::{*, Constraint::*}, widgets::{*, block::{*, Position}}, style::palette::*};

use crate::app::App;
use crate::player::*;
use crate::util::colors;

const TODO_HEADER_BG: Color = tailwind::BLUE.c950;
const SELECTED_STYLE_FG: Color = tailwind::BLUE.c300;
const TEXT_COLOR: Color = tailwind::SLATE.c200;
const COMPLETED_TEXT_COLOR: Color = tailwind::GREEN.c500;

impl App {
    pub fn render_customize_screen(&mut self, frame: &mut Frame, area: Rect) {
        // initialize the class menu 
        self.menus.customize.items.append("bruh", "1", Status::Selected);
        self.menus.customize.items.append("bruh", "2", Status::Selected);
        self.menus.customize.items.append("bruh", "3", Status::Selected);

        // set up the screen container layout
        let vertical = Layout::vertical([
            Constraint::Percentage(20),
            Constraint::Min(0),
            Constraint::Percentage(20),
        ]);
        let [header_area, rest_area, footer_area] = vertical.areas(area);

        let vertical = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);
        let [upper_item_list_area, lower_item_list_area] = vertical.areas(rest_area);

        let title = Line::from(Span::styled(
            "Choose your class",
            Style::default().fg(colors::MAIN).add_modifier(Modifier::BOLD),
        ));
        let controls = Title::from(Line::from(vec![
            Span::from("Quit"),
            Span::styled("<q> ", Style::default().fg(colors::HIGHLIGHT).add_modifier(Modifier::BOLD)),
            Span::from("Nav <↓↑>"),
        ]));
    
        // create the block (container)
        let block = Block::default()
            .title(title)
            .title(controls.position(Position::Bottom))
            .borders(Borders::ALL)
            .style(Style::default());
            
        let msg = vec![
            Line::from(vec![Span::from("")]),
            Line::from(vec![Span::from("")]),
            Line::from(vec![Span::from("")]),
            Line::from(vec![
                Span::styled("Press <Enter> to begin", Style::default().add_modifier(Modifier::ITALIC)),
            ]),
        ];
        // render the container
        // frame.render_widget(Paragraph::new("").block(block).alignment(Alignment::Center), rest_area);
        frame.render_widget(Paragraph::new(msg).block(block), rest_area);
        // self.menus.customize.render_player_classes(frame, upper_item_list_area);
        self.menus.customize.render_class_desc(frame, lower_item_list_area);
    }
}

pub struct Customize {
    pub items: ClassList,
}

impl Customize {
    pub fn new() -> Self {
        Customize {
            items: ClassList::new()
        }
    }
}

impl Customize {
    pub fn render_player_classes(&mut self, frame: &mut Frame, area: Rect) {
        // We create two blocks, one is for the header (outer) and the other is for list (inner).
        let outer_block = Block::new()
            .title_alignment(Alignment::Center)
            .title("TODO List")
            .fg(TEXT_COLOR);
        let inner_block = Block::new()
            .fg(TEXT_COLOR);

        // We get the inner area from outer_block. We'll use this area later to render the table.
        let outer_area = area;
        let inner_area = outer_block.inner(outer_area);

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .items
            .items
            .iter()
            .enumerate()
            .map(|(i, todo_item)| todo_item.to_list_item(i))
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let items = List::new(items)
            .block(inner_block)
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED)
                    .fg(SELECTED_STYLE_FG),
            )
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);
        
        frame.render_widget(items, area);
    }

    pub fn render_class_desc(&mut self, frame: &mut Frame, area: Rect) {
        // We get the info depending on the item's state.
        let info = if let Some(i) = self.items.state.selected() {
            match self.items.items[i].status {
                Status::Selected => format!("✓ DONE: {}", self.items.items[i].desc),
                Status::Unselected => format!("TODO: {}", self.items.items[i].desc),
            }
        } else {
            "Nothing to see here...".to_string()
        };

        // We show the list item's info under the list in this paragraph
        let outer_info_block = Block::new()
            .borders(Borders::NONE)
            .title_alignment(Alignment::Center)
            .title("TODO Info")
            .fg(TEXT_COLOR);
        let inner_info_block = Block::new()
            .borders(Borders::NONE)
            .padding(Padding::horizontal(1));

        // This is a similar process to what we did for list. outer_info_area will be used for
        // header inner_info_area will be used for the list info.
        let outer_info_area = area;
        let inner_info_area = outer_info_block.inner(outer_info_area);

        let info_paragraph = Paragraph::new(info)
            .block(inner_info_block)
            .fg(TEXT_COLOR)
            .wrap(Wrap { trim: false });

        frame.render_widget(info_paragraph, inner_info_area);
    }

    /// Changes the status of the selected list item
    pub fn change_status(&mut self) {
        if let Some(i) = self.items.state.selected() {
            self.items.items[i].status = match self.items.items[i].status {
                Status::Selected => Status::Unselected,
                Status::Unselected => Status::Selected,
            }
        }
    }
}

#[derive(Copy, Clone)]
enum Status {
    Selected,
    Unselected,
}

pub struct ClassListItem {
    class: String,
    desc: String,
    status: Status,
}

impl ClassListItem {
    fn new(class: &str, desc: &str, status: Status) -> Self {
        Self {
            class: class.to_string(),
            desc: desc.to_string(),
            status,
        }
    }
}

pub struct ClassList {
    state: ListState,
    items: Vec<ClassListItem>,
    last_selected: Option<usize>,
}

impl ClassList {
    pub fn with_items(items: Vec<(&str, &str, Status)>) -> Self {
        ClassList {
            state: ListState::default(),
            items: items
                .into_iter()
                .map(|(class, desc, status)| ClassListItem::new(class, desc, status))
                .collect(),
            last_selected: None,
        }
    }

    pub fn new() -> Self {
        ClassList {
            state: ListState::default(),
            items: Vec::new(),
            last_selected: None,
        }
    }

    pub fn append(&mut self, class: &str, desc: &str, status: Status) {
        self.items.push(ClassListItem::new(class, desc, status));
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        let offset = self.state.offset();
        self.last_selected = self.state.selected();
        self.state.select(None);
        *self.state.offset_mut() = offset;
    }
}

impl ClassListItem {
    pub fn to_list_item(&self, index: usize) -> ListItem {
        let line = match self.status {
            Status::Unselected => Line::styled(format!(" ☐ {}", self.class), TEXT_COLOR),
            Status::Selected => Line::styled(
                self.class.clone(), COMPLETED_TEXT_COLOR
            ),
        };

        ListItem::new(line)
    }
}


