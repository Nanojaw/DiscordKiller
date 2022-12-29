use crate::login_page::page::{InputMode, LoginPage};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

impl<'a> LoginPage<'a> {
    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, cursor_pos: Option<(u16, u16)>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(
                [
                    Constraint::Length(6), // Title
                    Constraint::Length(3), // Username
                    Constraint::Length(3), // Password
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
            .style(Style::default().fg(Color::Blue))
            .alignment(Alignment::Center);
        f.render_widget(paragraph, chunks[0]);

        let username = Paragraph::new(self.username_password[0].as_ref())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => match self.field_idx {
                    0 => Style::default().fg(Color::Yellow),
                    1 => Style::default(),

                    _ => Style::default(),
                },
            })
            .block(Block::default().borders(Borders::ALL).title("Username"));
        f.render_widget(username, chunks[1]);

        let password = Paragraph::new(self.password_stars.as_ref())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => match self.field_idx {
                    0 => Style::default(),
                    1 => Style::default().fg(Color::Yellow),

                    _ => Style::default(),
                },
            })
            .block(Block::default().borders(Borders::ALL).title("Password"));
        f.render_widget(password, chunks[2]);

        let (msg, style) = match self.input_mode {
            InputMode::Normal => (
                vec![
                    Span::raw("Press "),
                    Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to exit, "),
                    Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to start editing."),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
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
                Style::default(),
            ),
        };
        let mut text = Text::from(Spans::from(msg));
        text.patch_style(style);
        let help_message = Paragraph::new(text);
        f.render_widget(help_message, chunks[3]);

        match self.input_mode {
            InputMode::Normal =>
                // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
                {}

            InputMode::Editing => {
                // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
                if cursor_pos.unwrap().0
                    > chunks[self.field_idx + 1].x
                        + self.username_password[self.field_idx].chars().count() as u16
                        + 1
                {
                    f.set_cursor(
                        // Put cursor past the end of the input text
                        chunks[self.field_idx + 1].x
                            + self.username_password[self.field_idx].chars().count() as u16
                            + 1,
                        // Move one line down, from the border to the input line
                        chunks[self.field_idx + 1].y + 1,
                    )
                } else if cursor_pos.unwrap().0 <= chunks[self.field_idx + 1].x {
                    f.set_cursor(cursor_pos.unwrap().0 + 1, chunks[self.field_idx + 1].y + 1);
                } else {
                    f.set_cursor(cursor_pos.unwrap().0, chunks[self.field_idx + 1].y + 1);
                }
            }
        }
    }
}
