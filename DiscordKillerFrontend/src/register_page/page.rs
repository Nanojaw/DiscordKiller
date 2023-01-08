use crate::event::EventManager;

use reqwest::Client;
use tui_textarea::TextArea;

pub enum NextPage {
    CreateUser,
    Login,
    Quit,
}

pub enum SelectedWidget {
    EmailInput,
    RegisterButton,
    LoginLink,
}

pub struct RegisterPage<'a> {
    pub title: &'a str,

    pub http_client: Client,

    pub email_input: TextArea<'a>,

    pub event_manager: EventManager,

    pub selected_widget: SelectedWidget,

    pub should_quit: bool,
    pub should_redraw: bool,
    pub should_submit: bool,
    pub should_login: bool,
}

impl<'a> RegisterPage<'a> {
    pub fn new(title: &'a str) -> RegisterPage<'a> {
        RegisterPage {
            title,

            http_client: Client::new(),

            email_input: TextArea::default(),

            event_manager: EventManager::new(),

            selected_widget: SelectedWidget::EmailInput,

            should_quit: false,
            should_redraw: false,
            should_submit: false,
            should_login: false,
        }
    }
}
