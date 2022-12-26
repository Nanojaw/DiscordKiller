use crate::login::LoginApp;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    widgets::{
        Axis, BarChart, Block, Borders, Cell, Chart, Dataset, Gauge, LineGauge, List, ListItem,
        Paragraph, Row, Sparkline, Table, Tabs, Wrap,
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
    //draw_first_tab(f, app, chunks[1]);
}

fn draw_first_tab<B>(f: &mut Frame<B>, app: &mut LoginApp, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(9),
                Constraint::Min(8),
                Constraint::Length(7),
            ]
            .as_ref(),
        )
        .split(area);
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
