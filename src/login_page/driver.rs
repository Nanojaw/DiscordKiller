use super::page::{InputMode, LoginPage};
use crate::event::{Event, Key};
use std::{
    io::{Error, Result},
    ops::Deref,
    time::Duration,
    time::Instant,
};
use tui::{backend::Backend, Terminal};

impl<'a> LoginPage<'a> {
    pub fn handle_ce<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        ce: crossterm::event::Event,
    ) -> std::io::Result<()> {
        match ce {
            crossterm::event::Event::FocusGained => Ok(()),
            crossterm::event::Event::FocusLost => Ok(()),
            crossterm::event::Event::Key(ck) => {
                match self.input_mode {
                    InputMode::Normal => match Key::from(ck) {
                        Key::Ctrl('c') | Key::Char('q') => {
                            self.should_quit = true;
                            Ok(())
                        }

                        Key::Char('e') => {
                            self.input_mode = InputMode::Editing;

                            terminal.draw(|f| self.draw(f))?;

                            Ok(())
                        }
                        _ => Ok(()),
                    },
                    InputMode::Editing => match Key::from(ck) {
                        Key::Enter => {
                            self.field_idx += 1;

                            terminal.draw(|f| self.draw(f))?;

                            if self.field_idx == 2 {
                                // Submit to server and do stuff

                                self.should_quit = true;
                                return Ok(());
                            }

                            Ok(())
                        }
                        Key::Esc => {
                            self.input_mode = InputMode::Normal;

                            terminal.draw(|f| self.draw(f))?;

                            Ok(())
                        }
                        Key::Tab => {
                            self.field_idx = (self.field_idx + 1) % 2;

                            terminal.draw(|f| self.draw(f))?;

                            Ok(())
                        }
                        Key::Backspace => {
                            self.username_password[self.field_idx].pop();

                            if self.field_idx == 1 {
                                self.password_stars.pop();

                                terminal.draw(|f| self.draw(f))?;

                                return Ok(());
                            }

                            terminal.draw(|f| self.draw(f))?;

                            Ok(())
                        }
                        Key::Char(c) => {
                            self.username_password[self.field_idx].push(c);

                            if self.field_idx == 1 {
                                self.password_stars.push('*');

                                terminal.draw(|f| self.draw(f))?;

                                return Ok(());
                            }

                            terminal.draw(|f| self.draw(f))?;

                            Ok(())
                        }

                        _ => Ok(()),
                    },
                }
            }
            crossterm::event::Event::Mouse(cm) => Ok(()),
            crossterm::event::Event::Paste(cp) => Ok(()),
            crossterm::event::Event::Resize(width, height) => Ok(()),
        }
    }

    pub fn on_tick(&self) -> std::io::Result<()> {
        Ok(())
    }

    pub async fn run_app<B: Backend>(
        mut self,
        terminal: &mut Terminal<B>,
        tick_rate: Duration,
    ) -> std::io::Result<()> {
        terminal.draw(|f| self.draw(f))?;

        loop {
            let curr_event = self.event_manager.get(tick_rate).await;

            match curr_event {
                Event::CrosstermEvent(ce) => self.handle_ce(terminal, ce)?,
                Event::Tick => self.on_tick()?,
            }

            /*
            match nxt_event.is_ok() {
                true => match nxt_event.unwrap() {
                    Event::Input(key) => {
                        match self.input_mode {
                            InputMode::Editing => match key {


                                _ => {}
                            },
                        }
                    },
                    _ => {}
                },
                false => todo!(),
            }
            */
            //println!("{:?}", Instant::now() - now);
            /*
            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {

                }
            }
            */
            if self.should_quit {
                return Ok(());
            }
        }
    }
}
