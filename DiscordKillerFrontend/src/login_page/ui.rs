use crate::{
    login_page::page::{LoginPage, SelectedWidget},
    styles::{BUTTON, CURSOR, DEFAULT, HEADER, SELECTED, SELECTED_BUTTON},
};

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

impl<'a> LoginPage<'a> {
    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Ratio(1, 2), // Header
                Constraint::Length(6),   // Username-Password
                Constraint::Length(1),   // Login button
                Constraint::Min(1),      // Register
                Constraint::Length(1),   // Help message
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
                    SelectedWidget::UsernameInput => SELECTED,
                    _ => DEFAULT,
                }),
        );
        self.username_input.set_style(match self.selected_widget {
            SelectedWidget::UsernameInput => SELECTED,
            _ => DEFAULT,
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
                    SelectedWidget::PasswordInput => SELECTED,
                    _ => DEFAULT,
                }),
        );
        self.password_input.set_style(match self.selected_widget {
            SelectedWidget::PasswordInput => SELECTED,
            _ => DEFAULT,
        });
        self.password_input.set_cursor_line_style(Style::default());
        self.password_input
            .set_cursor_style(match self.selected_widget {
                SelectedWidget::PasswordInput => CURSOR,
                _ => Style::default(),
            });

        let input_field_layout = Layout::default()
            .horizontal_margin(25)
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(50), // Username
                    Constraint::Percentage(50), // Password
                ]
                .as_ref(),
            )
            .split(main_layout[1]);

        // Render the input fields
        f.render_widget(self.username_input.widget(), input_field_layout[0]);

        // Hide the password
        if self.hide_password {
            let mut hidden_input = self.password_input.clone();
            let char_count = hidden_input.lines().concat().chars().count();
            hidden_input.delete_str(0, char_count);
            hidden_input.insert_str("*".repeat(char_count));
            hidden_input.move_cursor(tui_textarea::CursorMove::Jump(self.password_input.cursor().0 as u16, self.password_input.cursor().1 as u16));
            f.render_widget(hidden_input.widget(), input_field_layout[1])
        } else {
            f.render_widget(self.password_input.widget(), input_field_layout[1]);
        }

        // Create the login button
        let login = Paragraph::new(Span::styled(
            "Login",
            match self.selected_widget {
                SelectedWidget::LoginButton => SELECTED_BUTTON,
                _ => BUTTON,
            },
        ))
        .alignment(Alignment::Center);
        f.render_widget(login, main_layout[2]);

        // Create the register link
        let register = Paragraph::new(Span::styled(
            "No account? Register!",
            match self.selected_widget {
                SelectedWidget::RegisterLink => SELECTED,
                _ => DEFAULT,
            },
        ))
        .alignment(Alignment::Center);
        f.render_widget(register, main_layout[3]);

        // Create the help message at the bottom
        let help_text = vec![Spans::from(
            "Press ctrl+q to quit, press ctrl+h to hide password",
        )];
        let help_message = Paragraph::new(help_text)
            .style(DEFAULT)
            .alignment(Alignment::Center);
        f.render_widget(help_message, main_layout[4]);
    }
}
