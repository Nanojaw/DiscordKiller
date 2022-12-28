use tui::{backend::Backend, Terminal};
use super::app::{LoginApp, InputMode};
use crate::event::{Events, Event, Key};
use std::{time::Duration, time::Instant, io::{Result, Error}};

impl<'a> LoginApp<'a> {
    pub fn run_app<B: Backend>(mut self, terminal: &mut Terminal<B>, tick_rate: Duration,
    ) -> std::io::Result<()> {
        let mut last_tick = Instant::now();
        loop {
            terminal.draw(|f| self.draw(f))?;
    
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            
            let nxt_event = self.events.next();

            match nxt_event.is_ok() {
                true => match nxt_event.unwrap() {
                    Event::Input(key) => {
                        match self.input_mode {
                            InputMode::Normal => match key {
                                Key::Char('e') => {
                                    self.input_mode = InputMode::Editing;
                                }
                                Key::Char('q') => {
                                    return Ok(());
                                }
                                
                                _ => {}
                            }
                            InputMode::Editing => match key {
                                Key::Enter => {
                                    self.field_idx += 1;
                                    
                                    if self.field_idx == 2 {
                                        // Submit to server and do stuff
                                    }
                                }
                                Key::Tab => {
                                    self.field_idx = (self.field_idx + 1) % 2;
                                }
                                Key::Char(c) => {
                                    self.username_password[self.field_idx].push(c);
                                    
                                    if self.field_idx == 1 {
                                        self.password_stars.push('*');
                                    }
                                }
                                Key::Backspace => {
                                    self.username_password[self.field_idx].pop();
                                    
                                    if self.field_idx == 1 {
                                        self.password_stars.pop();
                                    }
                                }
                                Key::Esc => {
                                    self.input_mode = InputMode::Normal
                                }
                                
                                _ => {}
                            },
                        }
                    },
                    Event::Tick => {
    
                    },
                },
                false => todo!(),
            }
            
                /*
            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    
                }
            }
            */
            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
            if self.should_quit {
                return Ok(());
            }
        }
    }
}