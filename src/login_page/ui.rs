use crate::{
    login_page::page::{InputMode, LoginPage},
    styles::{BORDER, BORDER_SELECTED, CURSOR, HEADER, TEXT, TEXT_SELECTED, HELP_MENU},
};
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

        let paragraph = Paragraph::new(text)
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

        self.input_fields[0].set_block(
            Block::default()
                .borders(Borders::all())
                .title("Username")
                .border_style(match self.input_mode {
                    InputMode::Normal => BORDER,
                    InputMode::Editing => match self.field_idx {
                        0 => BORDER_SELECTED,
                        1 => BORDER,

                        _ => BORDER,
                    },
                }),
        );
        self.input_fields[1].set_block(
            Block::default()
                .borders(Borders::all())
                .title("Password")
                .border_style(match self.input_mode {
                    InputMode::Normal => BORDER,
                    InputMode::Editing => match self.field_idx {
                        0 => BORDER,
                        1 => BORDER_SELECTED,

                        _ => BORDER,
                    },
                }),
        );

        self.input_fields[0].set_style(match self.input_mode {
            InputMode::Normal => TEXT,
            InputMode::Editing => match self.field_idx {
                0 => TEXT_SELECTED,
                1 => TEXT,

                _ => TEXT,
            },
        });
        self.input_fields[1].set_style(match self.input_mode {
            InputMode::Normal => TEXT,
            InputMode::Editing => match self.field_idx {
                0 => TEXT,
                1 => TEXT_SELECTED,

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

        if let Some(..) = field_cmd {
            self.input_fields[self.field_idx].input(field_cmd.unwrap());
        }

        f.render_widget(self.input_fields[0].widget(), username_password_chunks[0]);

        if self.render_stars {
            let mut password_stars_field = self.input_fields[1].clone();

            let original_cursor_pos = password_stars_field.cursor();

            password_stars_field.move_cursor(CursorMove::WordForward);

            for _ in 0..password_stars_field.lines()[0].chars().count() {
                password_stars_field.delete_char();
            }

            for _ in 0..self.input_fields[1].lines()[0].chars().count() {
                password_stars_field.insert_char('*');
            }

            password_stars_field.move_cursor(CursorMove::Jump(
                original_cursor_pos.0 as u16,
                original_cursor_pos.1 as u16,
            ));

            f.render_widget(password_stars_field.widget(), username_password_chunks[1])
        } else {
            f.render_widget(self.input_fields[1].widget(), username_password_chunks[1]);
        }

        let (msg, style) = match self.input_mode {
            InputMode::Normal => (
                vec![
                    Span::raw("Press "),
                    Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to exit, "),
                    Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to start editing."),
                ],
                HELP_MENU,
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
                HELP_MENU,
            ),
        };

        let mut text = Text::from(Spans::from(msg));
        text.patch_style(style);

        let mut full_message = vec![text.lines[0].clone()];

        if self.field_idx == 1 {
            let mut extra = Text::from(Spans::from(vec![
                Span::styled("Ctr+r", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to toggle stars"),
            ]));

            extra.patch_style(style);

            full_message.push(extra.lines[0].clone());
        }

        let help_message = Paragraph::new(full_message).alignment(Alignment::Center);

        f.render_widget(help_message, chunks[3]);
    }
}
