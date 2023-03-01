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

// Error
pub const ERROR: Style = Style {
    fg: Some(Color::Rgb(255, 255, 0)),
    bg: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};

pub const ERROR_TXT: Style = Style {
    fg: Some(Color::Rgb(253, 208, 23)),
    bg: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),    
};