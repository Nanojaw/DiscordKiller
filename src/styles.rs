use tui::style::{Color, Modifier, Style};

pub const TEXT: Style = Style {
    fg: Some(Color::Rgb(0, 150, 0)),
    bg: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};

pub const SELECTED_TEXT: Style = Style {
    fg: Some(Color::Rgb(180, 0, 0)),
    bg: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};
pub const HEADER: Style = Style {
    fg: Some(Color::Rgb(2, 181, 214)),
    bg: None,
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::empty(),
};
pub const CURSOR: Style = Style {
    fg: None,
    bg: Some(Color::Rgb(0, 0, 255)),
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};