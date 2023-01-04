use tui::style::{Color, Modifier, Style};

pub const HEADER: Style = Style {
    fg: Some(Color::Rgb(2, 181, 214)),
    bg: None,
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::empty(),
};
// Text

pub const DEFAULT: Style = Style {
    fg: Some(Color::Rgb(0, 150, 0)),
    bg: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};

pub const SELECTED: Style = Style {
    fg: Some(Color::Rgb(180, 0, 0)),
    bg: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};

// Cursor

pub const CURSOR: Style = Style {
    fg: None,
    bg: Some(Color::Rgb(255, 255, 255)),
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::empty(),
};

// Button

pub const BUTTON: Style = Style {
    fg: Some(Color::Black), // Set to terminal color black to mimic background
    bg: DEFAULT.fg,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};
pub const SELECTED_BUTTON: Style = Style {
    fg: Some(Color::Black), // Set to terminal color black to mimic background
    bg: SELECTED.fg,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};
