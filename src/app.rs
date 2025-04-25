use color_eyre::Result;
use ratatui::{
    Terminal,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout, Rect},
    prelude::CrosstermBackend,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, HighlightSpacing, List, ListItem, ListState, Paragraph, StatefulWidget,
        Widget,
    },
};
use ratatui_image::{StatefulImage, picker::Picker};

use crate::{
    applications::{self, Action, Application, get_app_icon, spawn_app},
    config::{Config, load_config},
    image::get_image,
};

pub struct App {
    input: String,
    character_index: usize,
    application_list: ApplicationList,
    original_list: Vec<Application>,
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
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .areas(area);

        let [list_area, item_area] =
            Layout::horizontal([Constraint::Fill(2), Constraint::Fill(1)]).areas(main_area);

        let [icon_area, about_area, action_area] = Layout::vertical([
            Constraint::Percentage(40),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .areas(item_area);
        self.render_header(header_area, buf);
        self.render_list(list_area, buf);
        self.render_selected_item(icon_area, about_area, action_area, buf);
        self.render_footer(footer_area, buf);
    }
}

impl App {
    pub fn new(config_path: Option<String>) -> Self {
        let apps = applications::get_apps();
        // Get the actions for the first application
        let info = if let Some(i) = apps.first() {
            i.clone()
        } else {
            Application {
                entry: "".to_string(),
                name: "No applications".to_string(),
                comment: "No applications found".to_string(),
                icon: "".to_string(),
                terminal: false,
                actions: vec![],
                categories: vec![],
            }
        };
        App {
            input: String::new(),
            character_index: 0,
            original_list: apps.clone(),
            application_list: ApplicationList {
                applications: apps,
                state: {
                    let mut state = ListState::default();
                    state.select(Some(0));
                    state
                },
            },
            action_list: ActionList {
                actions: info.actions.clone(),
                state: {
                    let mut state = ListState::default();
                    state.select(Some(0));
                    state
                },
            },
            config: load_config(config_path).unwrap_or_else(|_| {
                panic!("Failed to load config");
            }),
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

    fn select_next_app(&mut self) {
        let is_not_last_app = self.application_list.state.selected()
            != Some(self.application_list.applications.len() - 1);
        if is_not_last_app {
            self.application_list.state.select_next();
        } else {
            self.application_list.state.select(Some(0));
        }
        self.update_actions();
    }

    fn select_previous_app(&mut self) {
        let is_not_first_app = self.application_list.state.selected() != Some(0);
        if is_not_first_app {
            self.application_list.state.select_previous();
        } else {
            self.application_list
                .state
                .select(Some(self.application_list.applications.len() - 1));
        }
        self.update_actions();
    }

    fn select_next_action(&mut self) {
        let is_not_last_action =
            self.action_list.state.selected() != Some(self.action_list.actions.len() - 1);
        if is_not_last_action {
            self.action_list.state.select_next();
        } else {
            self.action_list.state.select(Some(0));
        }
    }

    fn select_previous_action(&mut self) {
        let is_not_first_action = self.action_list.state.selected() != Some(0);
        if is_not_first_action {
            self.action_list.state.select_previous();
        } else {
            self.action_list
                .state
                .select(Some(self.action_list.actions.len() - 1));
        }
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
            let new_input: String = before_char_to_delete.chain(after_char_to_delete).collect();
            self.update_input(new_input);
            self.move_cursor_left();
        }
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        let mut new_input = self.input.clone();
        new_input.insert(index, new_char);
        self.update_input(new_input);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    pub fn update_input(&mut self, new_input: String) {
        self.input = new_input;
        self.on_input_change();
    }

    fn update_actions(&mut self) {
        let info = if let Some(i) = self.application_list.state.selected() {
            self.application_list.applications[i].clone()
        } else {
            self.application_list.applications[0].clone()
        };
        self.action_list.actions = info.actions.clone();
        self.action_list.state.select(Some(0));
    }

    fn on_input_change(&mut self) {
        // Filter the applications based on the input
        let filtered_apps: Vec<Application> = self
            .original_list
            .iter()
            .filter(|app| app.name.to_lowercase().contains(&self.input.to_lowercase()))
            .cloned()
            .collect();
        if filtered_apps.is_empty() {
            let temp_app = Application {
                entry: "".to_string(),
                name: "No results".to_string(),
                comment: "No applications found".to_string(),
                icon: "".to_string(),
                terminal: false,
                actions: vec![Action {
                    name: "Try typing something else".to_string(),
                    command: "Or exit the application".to_string(),
                }],
                categories: vec![],
            };
            self.application_list.applications = vec![temp_app];
            self.application_list.state.select(Some(0));
            self.update_actions();

            return;
        }
        self.application_list.applications = filtered_apps;
        self.application_list.state.select(Some(0));
        self.update_actions();
    }

    fn run_action(&self) -> Result<()> {
        let selected_index = self.application_list.state.selected().unwrap_or(0);
        let selected_action_index = self.action_list.state.selected().unwrap_or(0);

        let selected_action =
            &self.application_list.applications[selected_index].actions[selected_action_index];
        let command = selected_action.command.clone();

        let is_terminal = self.application_list.applications[selected_index].terminal;

        spawn_app(command, is_terminal, &self.config)
    }

    fn run(mut self, terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
        loop {
            // terminal.draw(|frame| self.draw(frame))?;
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                        terminal.clear()?;
                        return Ok(());
                    }
                    KeyCode::Char(to_insert) => self.enter_char(to_insert),
                    KeyCode::Backspace => self.delete_char(),
                    KeyCode::Left => self.move_cursor_left(),
                    KeyCode::Right => self.move_cursor_right(),
                    KeyCode::Esc => {
                        terminal.clear()?;
                        return Ok(());
                    }
                    KeyCode::Enter => {
                        self.run_action()?;
                        terminal.clear()?;
                        return Ok(());
                    }
                    KeyCode::Up => self.select_previous_app(),
                    KeyCode::Down => self.select_next_app(),
                    KeyCode::Tab => self.select_next_action(),
                    KeyCode::BackTab => self.select_previous_action(),
                    _ => {}
                }
            }
        }
    }

    fn render_header(&self, header_area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.input.as_str())
            .fg(self.config.appearance.search_input)
            .block(
                Block::bordered()
                    .title("Search")
                    .fg(self.config.appearance.search_border)
                    .border_type(BorderType::Rounded),
            )
            .render(header_area, buf);
        buf.set_string(
            header_area.x + self.character_index as u16 + 1,
            header_area.y + 1,
            self.input
                .chars()
                .nth(self.character_index)
                .unwrap_or(' ')
                .to_string(),
            Style::default()
                .bg(self.config.appearance.search_input)
                .fg(Color::White),
        );
    }

    fn render_footer(&self, footer_area: Rect, buf: &mut Buffer) {
        Paragraph::new(
            "↑↓ to navigate apps | Tab to navigate actions | Enter to run action | Esc to exit",
        )
        .fg(self.config.appearance.help_text)
        .block(
            Block::bordered()
                .title("Controls")
                .fg(self.config.appearance.help_border)
                .border_type(BorderType::Rounded),
        )
        .render(footer_area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title("Applications")
            .border_type(BorderType::Rounded)
            .borders(ratatui::widgets::Borders::ALL)
            .fg(self.config.appearance.applications_border);
        let items: Vec<ListItem> = self
            .application_list
            .applications
            .iter()
            .map(|app| {
                let text = Text::from(vec![
                    Line::from(Span::styled(
                        app.name.clone(),
                        Style::default()
                            .fg(self.config.appearance.text)
                            .add_modifier(Modifier::BOLD),
                    )),
                    Line::from(Span::styled(
                        app.comment.clone(),
                        Style::default().fg(self.config.appearance.subtext),
                    )),
                ]);
                ListItem::new(text)
            })
            .collect();
        let final_list = List::new(items)
            .block(block)
            .highlight_style(
                Style::default()
                    .bg(self.config.appearance.selected_app)
                    .fg(self.config.appearance.selected_app_text),
            )
            .highlight_symbol(">> ")
            .highlight_spacing(HighlightSpacing::WhenSelected);
        StatefulWidget::render(final_list, area, buf, &mut self.application_list.state);
    }

    fn render_selected_item(
        &mut self,
        icon_area: Rect,
        about_area: Rect,
        action_area: Rect,
        buf: &mut Buffer,
    ) {
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
                    .fg(self.config.appearance.icon_border)
                    .border_type(BorderType::Rounded),
            );
            no_icon.render(icon_area, buf);
        } else {
            let picker = Picker::from_query_stdio().unwrap_or(Picker::from_fontsize((7, 14)));
            let dyn_img = get_image(icon_path.clone());
            let mut img = picker.new_resize_protocol(dyn_img.unwrap());
            Block::new()
                .title("Icon")
                .border_type(BorderType::Rounded)
                .borders(ratatui::widgets::Borders::ALL)
                .fg(self.config.appearance.icon_border)
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
                    .fg(self.config.appearance.text)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                info.comment.clone(),
                Style::default().fg(self.config.appearance.subtext),
            )),
            Line::from(Span::styled(
                info.categories.join(", "),
                Style::default().fg(self.config.appearance.subtext),
            )),
        ]);
        let selected_item = Paragraph::new(text).block(
            Block::bordered()
                .title("Info")
                .fg(self.config.appearance.info_border)
                .border_type(BorderType::Rounded),
        );
        selected_item.render(about_area, buf);

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
                            .fg(self.config.appearance.text)
                            .add_modifier(Modifier::BOLD),
                    )),
                    Line::from(Span::styled(
                        action.command.clone(),
                        Style::default().fg(self.config.appearance.subtext),
                    )),
                ]);
                ListItem::new(text)
            })
            .collect();
        let final_list = List::new(items)
            .block(block)
            .fg(self.config.appearance.actions_border)
            .highlight_style(
                Style::default()
                    .bg(self.config.appearance.selected_app)
                    .fg(Color::Black),
            )
            .highlight_symbol(">> ")
            .highlight_spacing(HighlightSpacing::WhenSelected);
        StatefulWidget::render(final_list, action_area, buf, &mut self.action_list.state);
    }
}

pub fn startup(config_path: Option<String>) -> Result<()> {
    color_eyre::install()?;
    let mut terminal: Terminal<CrosstermBackend<std::io::Stdout>> = ratatui::init();
    let app_result = App::new(config_path).run(&mut terminal);
    ratatui::restore();
    app_result
}
