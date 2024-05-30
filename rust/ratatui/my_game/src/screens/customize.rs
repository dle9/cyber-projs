use ratatui::{prelude::{*, Constraint::*}, widgets::{*, block::{*, Position}}, style::palette::*};

use crate::app::App;
use crate::player::{Player, Class};
use crate::util::colors;

const TODO_HEADER_BG: Color = tailwind::BLUE.c950;
const SELECTED_STYLE_FG: Color = tailwind::BLUE.c300;
const TEXT_COLOR: Color = tailwind::SLATE.c200;
const COMPLETED_TEXT_COLOR: Color = tailwind::GREEN.c500;

impl App {
    pub fn init_customize_screen(&mut self) {
        self.menus.customize.items.append("Serf", Player::new().get_skills(Class::Serf).as_str(), Status::Selected);
        self.menus.customize.items.append("Bibliosoph", Player::new().get_skills(Class::Bibliosoph).as_str(), Status::Unselected);
        self.menus.customize.items.append("Vagabond", Player::new().get_skills(Class::Vagabond).as_str(), Status::Unselected);
        self.menus.customize.items.append("Pariah", Player::new().get_skills(Class::Pariah).as_str(), Status::Unselected);
    }

    pub fn render_customize_screen(&mut self, frame: &mut Frame, area: Rect) {

        // set up the screen container layout
        let vertical = Layout::vertical([
            Constraint::Min(0),
            Constraint::Percentage(50),
            Constraint::Min(0),
        ]);
        let [top_area, content_container, bottom_area] = vertical.areas(area);

        let content = Layout::vertical([
            Constraint::Min(4),
            Constraint::Percentage(100),
        ]);
        let [nil, content_area] = content.areas(content_container);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(1),
                Constraint::Percentage(45),
                Constraint::Percentage(45),
            ])
            .split(content_area);
        let left_content_area = chunks[1];
        let right_content_area = chunks[2];
        
        // define block text (displayed along border)
        let title = Line::from(Span::styled(
            "Choose your class",
            Style::default().fg(colors::MAIN).add_modifier(Modifier::BOLD),
        ));
        let controls = Title::from(Line::from(vec![
            Span::from("Quit"),
            Span::styled("<q> ", Style::default().fg(colors::HIGHLIGHT).add_modifier(Modifier::BOLD)),
            Span::from("Nav"),
            Span::styled("<↓↑> ", Style::default().fg(colors::HIGHLIGHT).add_modifier(Modifier::BOLD)),
        ]));
    
        // create the block (container)
        let block = Block::default()
            .title(title)
            .title(controls.position(Position::Bottom))
            .borders(Borders::ALL)
            .style(Style::default());
        
        frame.render_widget(Paragraph::new("").block(block), content_container);
        self.menus.customize.render_player_classes(frame, left_content_area);
        self.menus.customize.render_class_desc(frame, right_content_area);
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
        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let player_classes: Vec<ListItem> = self.items.items
            .iter()
            .enumerate()
            .map(|(i, todo_item)| todo_item.to_list_item(i))
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let player_classes = List::new(player_classes)
            .block(block)
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED)
                    .fg(SELECTED_STYLE_FG),
            )
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        // render the top half (player classes)
        frame.render_widget(player_classes, area);
    }

    pub fn render_class_desc(&mut self, frame: &mut Frame, area: Rect) {
        // We get the info depending on the item's state.
        let desc = if let Some(i) = self.items.state.selected() {
            format!("{}", self.items.items[i].desc)
        } else {
            "".to_string()
        };

        let block = Block::default()
            .style(Style::default());

        // This is a similar process to what we did for list. outer_info_area will be used for
        // header inner_info_area will be used for the list desc.
        let outer_info_area = area;
        let inner_info_area = block.inner(outer_info_area);

        let info_paragraph = Paragraph::new(desc)
            .block(block)
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


