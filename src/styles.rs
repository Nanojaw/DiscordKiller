use tui::style::{Color, Modifier, Style};

pub const HEADER: Style = Style {
    fg: Some(Color::Rgb(2, 181, 214)),
    bg: None,
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::empty(),
};

// Border

pub const BORDER: Style = Style {
    fg: Some(Color::Rgb(64, 64, 64)),
    bg: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};
pub const BORDER_SELECTED: Style = Style {
    fg: Some(Color::Rgb(0, 0, 0)),
    bg: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};

// Text

pub const TEXT: Style = Style {
    fg: Some(Color::Rgb(252, 252, 250)),
    bg: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};

pub const TEXT_SELECTED: Style = Style {
    fg: Some(Color::Rgb(64, 64, 64)),
    bg: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};

pub const HELP_MENU: Style = Style {
    fg: Some(Color::Rgb(64, 64, 64)),
    bg: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};

// Cursor

pub const CURSOR: Style = Style {
    fg: None,
    bg: Some(Color::Rgb(0, 150, 0)),
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::empty(),
};
