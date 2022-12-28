use crate::event::Events;

pub enum InputMode {
    Normal,
    Editing,
}

pub struct LoginApp<'a> {
    pub title: &'a str,

    pub events: Events,

    pub input_mode: InputMode,
    pub field_idx: usize,

    pub username_password: [String; 2],
    pub password_stars: String,

    pub should_quit: bool,
    pub enhanced_graphics: bool,
}

impl<'a> LoginApp<'a> {
    pub fn new(title: &'a str, events: Events, enhanced_graphics: bool) -> LoginApp<'a> {
        LoginApp {
            title,

            events: events,

            input_mode: InputMode::Normal,
            field_idx: 0,

            username_password: ["".to_string(), "".to_string()],
            password_stars: String::new(),

            should_quit: false,
            enhanced_graphics,
        }
    }
}
