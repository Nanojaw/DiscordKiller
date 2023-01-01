use crate::event::EventManager;

use tui_textarea::TextArea;

pub enum SelectedWidget {
    UsernameInput,
    PasswordInput,
}

pub struct LoginPage<'a> {
    pub title: &'a str,

    pub username_input: TextArea<'a>,
    pub password_input: TextArea<'a>,

    pub event_manager: EventManager,

    pub selected_widget: SelectedWidget,

    pub hide_password: bool,
    pub should_quit: bool,
    pub should_redraw: bool,
    pub should_submit: bool,
}

impl<'a> LoginPage<'a> {
    pub fn new(title: &'a str) -> LoginPage<'a> {
        LoginPage {
            title,

            username_input: TextArea::default(),
            password_input: TextArea::default(),

            event_manager: EventManager::new(),

            selected_widget: SelectedWidget::UsernameInput,

            hide_password: true,
            should_quit: false,
            should_redraw: false,
            should_submit: false,
        }
    }
}
