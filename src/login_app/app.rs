pub enum InputMode {
    Normal,
    Editing,
}

pub struct LoginApp<'a> {
    pub title: &'a str,

    pub input_mode: InputMode,
    pub input_idx: usize,

    pub username_password: [String; 2],

    pub should_quit: bool,
    pub enhanced_graphics: bool,
}

impl<'a> LoginApp<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> LoginApp<'a> {
        LoginApp {
            title,

            input_mode: InputMode::Normal,
            input_idx: 0,

            username_password: ["".to_string(), "".to_string()],

            should_quit: false,
            enhanced_graphics,
        }
    }
}
