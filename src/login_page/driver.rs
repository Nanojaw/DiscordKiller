use super::page::{InputMode, LoginPage};
use crate::event::{Event, Key};
use std::time::Duration;
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

                            terminal.draw(|f| self.draw(f, None))?;

                            Ok(())
                        }
                        _ => Ok(()),
                    },
                    InputMode::Editing => match Key::from(ck) {
                        Key::Enter => {
                            self.field_idx += 1;

                            terminal.draw(|f| self.draw(f, None))?;

                            if self.field_idx == 2 {
                                // Submit to server and do stuff

                                self.should_quit = true;

                                return Ok(());
                            }

                            Ok(())
                        }
                        Key::Esc => {
                            self.input_mode = InputMode::Normal;

                            terminal.draw(|f| self.draw(f, None))?;

                            Ok(())
                        }
                        Key::Tab => {
                            self.field_idx = (self.field_idx + 1) % 2;

                            terminal.draw(|f| self.draw(f, None))?;

                            Ok(())
                        }
                        Key::Backspace => {
                            terminal.draw(|f| self.draw(f, Some(tui_textarea::Input::from(ck))))?;
                            Ok(())
                        }
                        Key::Char(_) => {
                            terminal.draw(|f| self.draw(f, Some(tui_textarea::Input::from(ck))))?;
                            Ok(())
                        }

                        Key::Left => {
                            terminal.draw(|f| self.draw(f, Some(tui_textarea::Input::from(ck))))?;
                            Ok(())
                        }

                        Key::Right => {
                            terminal.draw(|f| self.draw(f, Some(tui_textarea::Input::from(ck))))?;
                            Ok(())
                        }

                        _ => Ok(()),
                    },
                }
            }
            crossterm::event::Event::Mouse(_) => Ok(()),
            crossterm::event::Event::Paste(_) => Ok(()),
            crossterm::event::Event::Resize(_, _) => Ok(()),
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
        terminal.draw(|f| self.draw(f, None))?;

        loop {
            let curr_event = self.event_manager.get(tick_rate).await;

            match curr_event {
                Event::CrosstermEvent(ce) => self.handle_ce(terminal, ce)?,
                Event::Tick => self.on_tick()?,
            }

            if self.should_quit {
                return Ok(());
            }
        }
    }
}
