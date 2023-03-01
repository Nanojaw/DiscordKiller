use crate::{event::EventManager, UserFromServer};

use reqwest::Client;
use tui_textarea::TextArea;

pub enum NextPage {
    Register,
    Main(UserFromServer),
    Quit,
}

pub enum SelectedWidget {
    UsernameInput,
    PasswordInput,
    LoginButton,
    RegisterLink,
}

impl SelectedWidget {
    pub fn from_i32(n: i32) -> SelectedWidget {
        match n {
            0 => SelectedWidget::UsernameInput,
            1 => SelectedWidget::PasswordInput,
            2 => SelectedWidget::LoginButton,
            3 => SelectedWidget::RegisterLink,
            _ => { panic!("{n} is not a widget") }
        }
    }
}

pub struct LoginPage<'a> {
    pub title: &'a str,

    pub http_client: Client,

    pub username_input: TextArea<'a>,
    pub password_input: TextArea<'a>,

    pub event_manager: EventManager,

    pub error: bool,
    pub error_message: &'a str,

    pub selected_widget: SelectedWidget,
    pub selected_widget_counter: i32,

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

            error: false,
            error_message: "",

            selected_widget: SelectedWidget::UsernameInput,
            selected_widget_counter: 0,

            hide_password: true,
            should_quit: false,
            should_redraw: false,
            should_submit: false,
            should_register: false,
        }
    }

    pub fn add_selected_widget_counter(&mut self) {
        self.selected_widget_counter += 1;

        if self.selected_widget_counter == 4 {
            self.selected_widget_counter = 0;
        }
    }
    pub fn subtract_selected_widget_counter(&mut self) {
        self.selected_widget_counter -= 1;

        if self.selected_widget_counter == -1 {
            self.selected_widget_counter = 3;
        }
    }
}
