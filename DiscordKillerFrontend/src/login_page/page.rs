use crate::event::EventManager;

use reqwest::Client;
use tui_textarea::TextArea;

pub enum SelectedWidget {
    UsernameInput,
    PasswordInput,
    LoginButton,
    RegisterLink,
}

pub struct LoginPage<'a> {
    pub title: &'a str,

    pub http_client: Client,

    pub username_input: TextArea<'a>,
    pub password_input: TextArea<'a>,

    pub event_manager: EventManager,

    pub selected_widget: SelectedWidget,

    pub hide_password: bool,
    pub should_quit: bool,
    pub should_redraw: bool,
    pub should_submit: bool,
    pub should_register: bool,
}

impl<'a> LoginPage<'a> {
    pub fn new(title: &'a str) -> LoginPage<'a> {
        LoginPage {
            title,

            http_client: Client::new(),

            username_input: TextArea::default(),
            password_input: TextArea::default(),

            event_manager: EventManager::new(),

            selected_widget: SelectedWidget::UsernameInput,

            hide_password: true,
            should_quit: false,
            should_redraw: false,
            should_submit: false,
            should_register: false
        }
    }
}
