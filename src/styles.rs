use tui::style::{Color, Modifier, Style};

pub const TEXT_STYLE: Style = Style {
    fg: Some(Color::LightGreen),
    bg: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};

pub const HEADER_STYLE: Style = Style {
    fg: Some(Color::LightCyan),
    bg: None,
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::empty(),
};