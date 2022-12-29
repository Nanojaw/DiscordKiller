use crate::event::EventManager;
use tui::{backend::Backend, Terminal};

pub enum InputMode {
    Normal,
    Editing,
}

pub struct LoginPage<'a, C: Backend> {
    pub title: &'a str,

    pub terminal: Terminal<C>,

    pub event_manager: EventManager,

    pub input_mode: InputMode,
    pub field_idx: usize,

    pub username_password: [String; 2],
    pub password_stars: String,

    pub should_quit: bool,
}

impl<'a, C: Backend> LoginPage<'a, C> {
    pub fn new(title: &'a str, term: Terminal<C>) -> LoginPage<'a, C> {
        LoginPage {
            title,

            terminal: term,

            event_manager: EventManager::new(),

            input_mode: InputMode::Normal,
            field_idx: 0,

            username_password: ["".to_string(), "".to_string()],
            password_stars: String::new(),

            should_quit: false,
        }
    }
}
