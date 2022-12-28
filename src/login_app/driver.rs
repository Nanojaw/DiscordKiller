use tui::{backend::Backend, Terminal};
use super::app::{LoginApp, InputMode};
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
                    match self.input_mode {
                        InputMode::Normal => match key.code {
                            KeyCode::Char('e') => {
                                self.input_mode = InputMode::Editing;
                            }
                            KeyCode::Char('q') => {
                                return Ok(());
                            }

                            _ => {}
                        }
                        InputMode::Editing => match key.code {
                            KeyCode::Enter => {
                                self.input_idx += 1;

                                if self.input_idx == 3 {
                                    // Submit to server and do stuff
                                }
                            }
                            KeyCode::Tab => {
                                self.input_idx = (self.input_idx + 1) % 2;
                            }
                            KeyCode::Char(c) => {
                                self.username_password[self.input_idx].push(c);
                            }
                            KeyCode::Backspace => {
                                self.username_password[self.input_idx].pop();
                            }
                            KeyCode::Esc => {
                                self.input_mode = InputMode::Normal
                            }

                            _ => {}
                        },
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