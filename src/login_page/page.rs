use crate::event::EventManager;

pub enum InputMode {
    Normal,
    Editing,
}

pub struct LoginPage<'a> {
    pub title: &'a str,

    pub event_manager: EventManager,

    pub input_mode: InputMode,
    pub field_idx: usize,

    pub username_password: [String; 2],
    pub password_stars: String,

    pub should_quit: bool,
}

impl<'a> LoginPage<'a> {
    pub fn new(title: &'a str) -> LoginPage<'a> {
        LoginPage {
            title,

            event_manager: EventManager::new(),

            input_mode: InputMode::Normal,
            field_idx: 0,

            username_password: ["".to_string(), "".to_string()],
            password_stars: String::new(),

            should_quit: false,
        }
    }
}
