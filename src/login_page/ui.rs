use crate::{
    login_page::page::{InputMode, LoginPage},
    styles::{CURSOR, HEADER, SELECTED_TEXT, TEXT},
};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use tui_textarea::Input;

impl<'a> LoginPage<'a> {
    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, field_cmd: Option<Input>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(6), // Title
                    Constraint::Length(6), // Username - Password fields
                    Constraint::Length(1), // Margin
                    Constraint::Length(1), // Help menu
                ]
                .as_ref(),
            )
            .split(f.size());

        let text = vec![
            Spans::from(r"                  _        _            "),
            Spans::from(r"|   |            | |      | |           "),
            Spans::from(r"|___|  __   __   | |  __  | |    __  _|_"),
            Spans::from(r"|   | /  | /     |/_)/    |/ \  /  |  | "),
            Spans::from(r"|   |/\_/|_\___//| \_\___/|   |/\_/|_ |_"),
        ];

        let paragraph = Paragraph::new(text.clone())
            .style(HEADER)
            .alignment(Alignment::Center);
        f.render_widget(paragraph, chunks[0]);

        let username_password_chunks = Layout::default()
            .horizontal_margin(25)
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(50), // Username
                    Constraint::Percentage(50), // Password
                ]
                .as_ref(),
            )
            .split(chunks[1]);

        self.input_fields[0].set_block(Block::default().borders(Borders::all()).title("Username"));
        self.input_fields[1].set_block(Block::default().borders(Borders::all()).title("Password"));

        self.input_fields[0].set_style(match self.input_mode {
            InputMode::Normal => TEXT,
            InputMode::Editing => match self.field_idx {
                0 => SELECTED_TEXT,
                1 => TEXT,

                _ => TEXT,
            },
        });
        self.input_fields[1].set_style(match self.input_mode {
            InputMode::Normal => TEXT,
            InputMode::Editing => match self.field_idx {
                0 => TEXT,
                1 => SELECTED_TEXT,

                _ => TEXT,
            },
        });

        self.input_fields[0].set_cursor_line_style(Style::default());
        self.input_fields[1].set_cursor_line_style(Style::default());

        self.input_fields[0].set_cursor_style(match self.input_mode {
            InputMode::Normal => TEXT,
            InputMode::Editing => match self.field_idx {
                0 => CURSOR,
                1 => Style::default(),

                _ => Style::default(),
            },
        });
        self.input_fields[1].set_cursor_style(match self.input_mode {
            InputMode::Normal => TEXT,
            InputMode::Editing => match self.field_idx {
                0 => Style::default(),
                1 => CURSOR,

                _ => Style::default(),
            },
        });

        if field_cmd.is_some() {
            self.input_fields[self.field_idx].input(field_cmd.unwrap());
        }

        f.render_widget(self.input_fields[0].widget(), username_password_chunks[0]);
        f.render_widget(self.input_fields[1].widget(), username_password_chunks[1]);

        let (msg, style) = match self.input_mode {
            InputMode::Normal => (
                vec![
                    Span::raw("Press "),
                    Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to exit, "),
                    Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to start editing."),
                ],
                TEXT,
            ),
            InputMode::Editing => (
                vec![
                    Span::raw("Press "),
                    Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to stop editing, "),
                    Span::styled("Tab", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to switch fields, "),
                    Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to move to the next field or submit"),
                ],
                TEXT,
            ),
        };
        let mut text = Text::from(Spans::from(msg));
        text.patch_style(style);
        let help_message = Paragraph::new(text).alignment(Alignment::Center);
        f.render_widget(help_message, chunks[3]);
    }
}
