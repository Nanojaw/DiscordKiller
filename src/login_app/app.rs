pub struct LoginApp<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub enhanced_graphics: bool,
}

impl<'a> LoginApp<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> LoginApp<'a> {
        LoginApp {
            title,
            should_quit: false,
            enhanced_graphics,
        }
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }
}