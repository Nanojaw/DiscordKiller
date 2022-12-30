use crate::event::EventManager;

use tui_textarea::TextArea;

pub enum InputMode {
    Normal,
    Editing,
}

pub struct LoginPage<'a> {
    pub title: &'a str,

    pub input_fields: [TextArea<'a>; 2],

    pub event_manager: EventManager,

    pub input_mode: InputMode,
    pub field_idx: usize,

    pub first_draw: bool,
    pub should_quit: bool,
}

impl<'a> LoginPage<'a> {
    pub fn new(title: &'a str) -> LoginPage<'a> {
        LoginPage {
            title,

            input_fields: [TextArea::default(), TextArea::default()],

            event_manager: EventManager::new(),

            input_mode: InputMode::Normal,
            field_idx: 0,

            first_draw: true,
            should_quit: false,
        }
    }
}
