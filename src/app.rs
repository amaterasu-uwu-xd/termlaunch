use color_eyre::Result;
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, HighlightSpacing, List, ListItem, ListState, Paragraph, StatefulWidget,
        Widget,
    },
};
use ratatui_image::{StatefulImage, picker::Picker};

use crate::{
    applications::{self, Action, Application, get_app_icon},
    config::{Config, load_config},
    image::get_image,
};

/// Estado de la aplicación
pub struct App {
    input: String,
    character_index: usize,
    application_list: ApplicationList,
    action_list: ActionList,
    config: Config,
}

struct ApplicationList {
    applications: Vec<Application>,
    state: ListState,
}

struct ActionList {
    actions: Vec<Action>,
    state: ListState,
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Max(10),
            Constraint::Min(1),
            Constraint::Max(10),
        ])
        .areas(area);

        let [list_area, item_area] =
            Layout::horizontal([Constraint::Fill(2), Constraint::Fill(1)]).areas(main_area);

        let [icon_area, about_area, actions_area] = Layout::vertical([
            Constraint::Percentage(40),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .areas(item_area);
        self.render_header(header_area, buf);
        // App::render_footer(footer_area, buf);
        self.render_list(list_area, buf);
        self.render_selected_item(icon_area, about_area, actions_area, buf);
    }
}

impl App {
    pub fn new(config_path: Option<String>) -> Self {
        App {
            input: String::new(),
            character_index: 0,
            application_list: ApplicationList {
                applications: applications::get_apps(),
                state: {
                    let mut state = ListState::default();
                    state.select(Some(0));
                    state
                },
            },
            action_list: ActionList {
                actions: Vec::new(),
                state: {
                    let mut state = ListState::default();
                    state.select(Some(0));
                    state
                },
            },
            config: load_config(config_path),
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn select_next(&mut self) {
        self.application_list.state.select_next();
    }

    fn select_next_action(&mut self) {
        self.action_list.state.select_next();
    }

    fn select_previous(&mut self) {
        self.application_list.state.select_previous();
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            // terminal.draw(|frame| self.draw(frame))?;
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(to_insert) => self.enter_char(to_insert),
                    KeyCode::Backspace => self.delete_char(),
                    KeyCode::Left => self.move_cursor_left(),
                    KeyCode::Right => self.move_cursor_right(),
                    KeyCode::Esc => return Ok(()),
                    KeyCode::Enter => todo!(),
                    KeyCode::Up => self.select_previous(),
                    KeyCode::Down => self.select_next(),
                    KeyCode::Tab => self.select_next_action(),
                    KeyCode::Delete => todo!(),
                    _ => {}
                }
            }
        }
    }

    fn render_header(&self, header_area: Rect, buf: &mut Buffer) {
        // let input = Paragraph::new(self.input.as_str())
        //     .block(Block::bordered().title("Input").border_type(BorderType::Rounded));
        // frame.render_widget(input, input_area);

        // frame.set_cursor_position(Position::new(
        //     input_area.x + self.character_index as u16 + 1,
        //     input_area.y + 1,
        // ));

        Paragraph::new(self.input.as_str())
            .block(
                Block::bordered()
                    .title("Search")
                    .border_type(BorderType::Rounded),
            )
            .render(header_area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title("Applications")
            .border_type(BorderType::Rounded)
            .borders(ratatui::widgets::Borders::ALL);
        let items: Vec<ListItem> = self
            .application_list
            .applications
            .iter()
            .map(|app| {
                let text = Text::from(vec![
                    Line::from(Span::styled(
                        app.name.clone(),
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    )),
                    Line::from(Span::styled(
                        app.comment.clone(),
                        Style::default().fg(Color::Gray),
                    )),
                ]);
                ListItem::new(text)
            })
            .collect();
        let final_list = List::new(items)
            .block(block)
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
            .highlight_symbol(">> ")
            .highlight_spacing(HighlightSpacing::Always);
        StatefulWidget::render(final_list, area, buf, &mut self.application_list.state);
    }

    fn render_selected_item(
        &mut self,
        icon_area: Rect,
        about_area: Rect,
        action_area: Rect,
        buf: &mut Buffer,
    ) -> () {
        let info = if let Some(i) = self.application_list.state.selected() {
            self.application_list.applications[i].clone()
        } else {
            self.application_list.applications[0].clone()
        };

        let icon_path = get_app_icon(info.icon, &self.config).unwrap_or_default();
        if icon_path.to_str().unwrap().is_empty() {
            let text = Text::from(vec![Line::from(Span::styled(
                "No icon available",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ))]);
            let no_icon = Paragraph::new(text).block(
                Block::bordered()
                    .title("Icon")
                    .border_type(BorderType::Rounded),
            );
            no_icon.render(icon_area, buf);
        } else {
            let picker = Picker::from_query_stdio().unwrap();
            let dyn_img = get_image(icon_path.clone());
            let mut img = picker.new_resize_protocol(dyn_img.unwrap());
            Block::new()
                .title("Icon")
                .border_type(BorderType::Rounded)
                .borders(ratatui::widgets::Borders::ALL)
                .render(icon_area, buf);
            StatefulWidget::render(
                StatefulImage::default(),
                Block::new()
                    .title("Icon")
                    .border_type(BorderType::Rounded)
                    .borders(ratatui::widgets::Borders::ALL)
                    .inner(icon_area),
                buf,
                &mut img,
            );
        }

        let text = Text::from(vec![
            Line::from(Span::styled(
                info.name.clone(),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                info.comment.clone(),
                Style::default().fg(Color::Gray),
            )),
            Line::from(Span::styled(
                info.categories.join(", "),
                Style::default().fg(Color::Gray),
            )),
        ]);
        let selected_item = Paragraph::new(text).block(
            Block::bordered()
                .title("Info")
                .border_type(BorderType::Rounded),
        );
        selected_item.render(about_area, buf);

        // Set the actual actions
        self.action_list.actions = info.actions.clone();
        self.action_list.state.select(Some(0));

        let block = Block::new()
            .title("Actions")
            .border_type(BorderType::Rounded)
            .borders(ratatui::widgets::Borders::ALL);
        let items: Vec<ListItem> = self
            .action_list
            .actions
            .iter()
            .map(|action| {
                let text = Text::from(vec![
                    Line::from(Span::styled(
                        action.name.clone(),
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    )),
                    Line::from(Span::styled(
                        action.command.clone(),
                        Style::default().fg(Color::Gray),
                    )),
                ]);
                ListItem::new(text)
            })
            .collect();
        let final_list = List::new(items)
            .block(block)
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
            .highlight_symbol(">> ")
            .highlight_spacing(HighlightSpacing::Always);
        StatefulWidget::render(final_list, action_area, buf, &mut self.action_list.state);
    }
}

pub fn startup(config_path: Option<String>) -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new(config_path).run(terminal);
    ratatui::restore();
    app_result
}
