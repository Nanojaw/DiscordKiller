use super::page::{RegisterPage, SelectedWidget};
use crate::styles::{CURSOR, DEFAULT, HEADER, SELECTED};

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};


impl<'a> RegisterPage<'a> {
    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Ratio(1, 2), // Header
            Constraint::Length(3),   // Email
            Constraint::Length(1),   // Register
            Constraint::Min(1),      // Login
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
        self.email_input.set_block(
            Block::default()
                .borders(Borders::all())
                .title("Email")
                .border_style(match self.selected_widget {
                    SelectedWidget::EmailInput => SELECTED,
                    _ => DEFAULT,
                }),
        );
        self.email_input.set_style(match self.selected_widget {
            SelectedWidget::EmailInput => SELECTED,
            _ => DEFAULT,
        });
        self.email_input.set_cursor_line_style(Style::default());
        self.email_input
            .set_cursor_style(match self.selected_widget {
                SelectedWidget::EmailInput => CURSOR,
                _ => Style::default(),
            });

            let input_field_layout = Layout::default()
            .horizontal_margin(25)
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(100) // Email
                ]
                .as_ref(),
            )
            .split(main_layout[1]);

        // Render the input fields
        f.render_widget(self.email_input.widget(), input_field_layout[0]);

         // Create the register button
         let login = Paragraph::new(Span::styled(
            "Register",
            match self.selected_widget {
                SelectedWidget::RegisterButton => SELECTED,
                _ => DEFAULT,
            },
        ))
        .alignment(Alignment::Center);
        f.render_widget(login, main_layout[2]);

        // Create the login link
        let register = Paragraph::new(Span::styled(
            "Alreay have an account? Login!",
            match self.selected_widget {
                SelectedWidget::LoginLink => SELECTED,
                _ => DEFAULT,
            },
        ))
        .alignment(Alignment::Center);
        f.render_widget(register, main_layout[3]);

        // Create the help message at the bottom
        let help_text = vec![Spans::from(
            "Press ctrl+q to quit",
        )];
        let help_message = Paragraph::new(help_text)
            .style(DEFAULT)
            .alignment(Alignment::Center);
        f.render_widget(help_message, main_layout[4]);
    }
}
