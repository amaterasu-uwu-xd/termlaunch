use color_eyre::Result;
use ratatui::{
    buffer::Buffer, crossterm::event::{self, Event, KeyCode, KeyEventKind}, layout::{Constraint, Layout, Position, Rect}, style::{Color, Modifier, Style, Stylize}, text::{Line, Span, Text}, widgets::{Block, BorderType, HighlightSpacing, List, ListItem, ListState, Paragraph, StatefulWidget, Widget}, DefaultTerminal, Frame
};
use ratatui_image::{picker::Picker, protocol::StatefulProtocol, Resize, StatefulImage};

use crate::{applications::{self, Application}, config::{load_config, Config}};

/// Estado de la aplicación
pub struct App {
    input: String,
    character_index: usize,
    application_list: ApplicationList,
    config: Config,
}

struct ApplicationList {
    applications: Vec<Application>,
    state: ListState,
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .areas(area);

        let [list_area, item_area] =
            Layout::horizontal([Constraint::Fill(2), Constraint::Fill(1)]).areas(main_area);

        self.render_header(header_area, buf);
        // App::render_footer(footer_area, buf);
        self.render_list(list_area, buf);
        self.render_selected_item(item_area, buf);
    }
}

impl App {
    pub fn new(config_path: Option<String>) -> Self {
        App {
            input: String::new(),
            character_index: 0,
            application_list: ApplicationList {
                applications: applications::get_apps(),
                state: ListState::default(),
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
                match key.code{
                    KeyCode::Char(to_insert) => self.enter_char(to_insert),
                    KeyCode::Backspace => self.delete_char(),
                    KeyCode::Left => self.move_cursor_left(),
                    KeyCode::Right => self.move_cursor_right(),
                    KeyCode::Esc => return Ok(()),
                    KeyCode::Enter => todo!(),
                    KeyCode::Up => self.select_previous(),
                    KeyCode::Down => self.select_next(),
                    KeyCode::Tab => todo!(),
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
            .block(Block::bordered()
            .title("Search")
            .border_type(BorderType::Rounded))
            .render(header_area, buf);
    }
    
    fn render_list(&mut self, list_area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title("Applications")
            .border_type(BorderType::Rounded)
            .borders(ratatui::widgets::Borders::ALL);
        let items: Vec<ListItem> = self.application_list.applications.iter().map(|app| {
            let text = Text::from(vec![
                Line::from(Span::styled(
                    app.name.clone(),
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                )),
                Line::from(Span::styled(
                    app.comment.clone(),
                    Style::default().fg(Color::Gray),
                )),
            ]);
            ListItem::new(text)
        }).collect();
        let final_list = List::new(items)
            .block(block)
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
            .highlight_symbol(">> ")
            .highlight_spacing(HighlightSpacing::Always);
        StatefulWidget::render(final_list, list_area, buf, &mut self.application_list.state);

    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        let info = if let Some(i) = self.application_list.state.selected() {
            self.application_list.applications[i].clone()
        } else {
            self.application_list.applications[1].clone()
        };
        let text = Text::from(vec![
            Line::from(Span::styled(
                info.name.clone(),
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                info.command.clone(),
                Style::default().fg(Color::Gray),
            )),
        ]);
        let selected_item = Paragraph::new(text)
            .block(Block::bordered()
            .title("Info")
            .border_type(BorderType::Rounded));
        selected_item.render(area, buf);
    }
}

pub fn startup(config_path: Option<String>) -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new(config_path).run(terminal);
    ratatui::restore();
    app_result
}
