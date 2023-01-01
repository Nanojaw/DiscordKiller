use crate::{
    login_page::page::{LoginPage, SelectedWidget},
    styles::{BORDER, BORDER_SELECTED, CURSOR, HEADER, HELP_MENU, TEXT, TEXT_SELECTED},
};
use reqwest::header;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use tui_textarea::{CursorMove, Input};

impl<'a> LoginPage<'a> {
    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(6),    // Header
                Constraint::Length(3), // Username
                Constraint::Length(3), // Password
            ])
            .split(f.size());

        // Don't display the header if we can't fit it
        if main_layout[0].height >= 6 {
            let header_text = vec![
                Spans::from(r"                  _        _            "),
                Spans::from(r"|   |            | |      | |           "),
                Spans::from(r"|___|  __   __   | |  __  | |    __  _|_"),
                Spans::from(r"|   | /  | /     |/_)/    |/ \  /  |  | "),
                Spans::from(r"|   |/\_/|_\___//| \_\___/|   |/\_/|_ |_"),
                Spans::from(r"                                        "),
            ];
            let header_container = Layout::default()
                .direction(Direction::Vertical)
                .vertical_margin((main_layout[0].height - header_text.len() as u16) / 2)
                .constraints([Constraint::Length(6)])
                .split(main_layout[0]);
            let header = Paragraph::new(header_text)
                .style(HEADER)
                .alignment(Alignment::Center);
            f.render_widget(header, header_container[0]);
        }

        // Configure the input fields
        self.username_input.set_block(
            Block::default()
                .borders(Borders::all())
                .title("Username")
                .border_style(match self.selected_widget {
                    SelectedWidget::UsernameInput => BORDER_SELECTED,
                    _ => BORDER,
                }),
        );
        self.username_input.set_style(match self.selected_widget {
            SelectedWidget::UsernameInput => TEXT_SELECTED,
            _ => TEXT,
        });
        self.username_input.set_cursor_line_style(Style::default());
        self.username_input
            .set_cursor_style(match self.selected_widget {
                SelectedWidget::UsernameInput => CURSOR,
                _ => Style::default(),
            });

        self.password_input.set_block(
            Block::default()
                .borders(Borders::all())
                .title("Password")
                .border_style(match self.selected_widget {
                    SelectedWidget::PasswordInput => BORDER_SELECTED,
                    _ => BORDER,
                }),
        );
        self.password_input.set_style(match self.selected_widget {
            SelectedWidget::PasswordInput => TEXT_SELECTED,
            _ => TEXT,
        });
        self.password_input.set_cursor_line_style(Style::default());
        self.password_input
            .set_cursor_style(match self.selected_widget {
                SelectedWidget::PasswordInput => CURSOR,
                _ => Style::default(),
            });

        // Render the input fields
        f.render_widget(self.username_input.widget(), main_layout[1]);

        // Hide the password
        if self.hide_password {
            let mut hidden_input = self.password_input.clone();
            let char_count = hidden_input.lines().concat().chars().count();
            hidden_input.delete_str(0, char_count);
            hidden_input.insert_str("*".repeat(char_count));
            f.render_widget(hidden_input.widget(), main_layout[2])
        } else {
            f.render_widget(self.password_input.widget(), main_layout[2]);
        }
    }
}
