use crate::login_app::app::{InputMode, LoginApp};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn draw_login_app<B: Backend>(f: &mut Frame<B>, app: &mut LoginApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1), // Title
                Constraint::Length(3), // Username
                Constraint::Length(3), // Password
                Constraint::Length(1), // Help menu
            ]
            .as_ref(),
        )
        .split(f.size());

    let username = Paragraph::new(app.username_password[0].as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => match app.input_idx {
                0 => Style::default().fg(Color::Yellow),
                1 => Style::default(),

                _ => Style::default(),
            },
        })
        .block(Block::default().borders(Borders::ALL).title("Username"));
    f.render_widget(username, chunks[1]);

    let password = Paragraph::new(app.username_password[1].as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => match app.input_idx {
                0 => Style::default(),
                1 => Style::default().fg(Color::Yellow),

                _ => Style::default(),
            },
        })
        .block(Block::default().borders(Borders::ALL).title("Password"));
    f.render_widget(password, chunks[2]);

    let (msg, style) = match app.input_mode {
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

    match app.input_mode {
        InputMode::Normal =>
        // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
        {}

        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[app.input_idx + 1].x + app.username_password[app.input_idx].len() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[app.input_idx + 1].y + 1,
            )
        }
    }
}
