use tui::{backend::Backend, Terminal};
use super::app::LoginApp;
use std::{time::Duration, time::Instant, io};
use crate::ui;
use crossterm::{event, event::Event, event::KeyCode};

impl<'a> LoginApp<'a> {
    pub fn run_app<B: Backend>(mut self, terminal: &mut Terminal<B>, tick_rate: Duration,
    ) -> io::Result<()> {
        let mut last_tick = Instant::now();
        loop {
            terminal.draw(|f| ui::draw_login_app(f, &mut self))?;
    
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(c) => self.on_key(c),
                        _ => {}
                    }
                }
            }
            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
            if self.should_quit {
                return Ok(());
            }
        }
    }
}