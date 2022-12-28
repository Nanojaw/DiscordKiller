use tui::{backend::Backend, Terminal};
use super::app::{LoginApp, InputMode};
use std::{time::Duration, time::Instant, io};
use crossterm::{event, event::Event, event::KeyCode};

impl<'a> LoginApp<'a> {
    pub fn run_app<B: Backend>(mut self, terminal: &mut Terminal<B>, tick_rate: Duration,
    ) -> io::Result<()> {
        let mut last_tick = Instant::now();
        loop {
            terminal.draw(|f| self.draw(f))?;
    
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
                                self.field_idx += 1;

                                if self.field_idx == 2 {
                                    // Submit to server and do stuff
                                }
                            }
                            KeyCode::Tab => {
                                self.field_idx = (self.field_idx + 1) % 2;
                            }
                            KeyCode::Char(c) => {
                                self.username_password[self.field_idx].push(c);
                                
                                if self.field_idx == 1 {
                                    self.password_stars.push('*');
                                }
                            }
                            KeyCode::Backspace => {
                                self.username_password[self.field_idx].pop();

                                if self.field_idx == 1 {
                                    self.password_stars.pop();
                                }
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