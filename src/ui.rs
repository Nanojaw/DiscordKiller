use crate::login_app::app::LoginApp;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, Borders,
        Paragraph, Wrap,
    },
    Frame,
};

pub fn draw_login_app<B: Backend>(f: &mut Frame<B>, app: &mut LoginApp) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    draw_text(f, chunks[0], vec![
        Spans::from("Login screen"),
    ]);
}

fn draw_text<B>(f: &mut Frame<B>, area: Rect, text: Vec<Spans>)
where
    B: Backend,
{
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Title",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    )).title_alignment(tui::layout::Alignment::Center);
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}
