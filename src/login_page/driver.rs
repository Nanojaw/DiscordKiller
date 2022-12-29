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
        let mut cursor_pos = terminal.get_cursor().unwrap();
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

                            terminal.draw(|f| self.draw(f, Some(cursor_pos)))?;

                            Ok(())
                        }
                        _ => Ok(()),
                    },
                    InputMode::Editing => match Key::from(ck) {
                        Key::Enter => {
                            self.field_idx += 1;

                            terminal.draw(|f| self.draw(f, Some(cursor_pos)))?;

                            if self.field_idx == 2 {
                                // Submit to server and do stuff

                                self.should_quit = true;
                                return Ok(());
                            }

                            Ok(())
                        }
                        Key::Esc => {
                            self.input_mode = InputMode::Normal;

                            terminal.draw(|f| self.draw(f, Some(cursor_pos)))?;

                            Ok(())
                        }
                        Key::Tab => {
                            self.field_idx = (self.field_idx + 1) % 2;

                            terminal.draw(|f| self.draw(f, Some(cursor_pos)))?;

                            Ok(())
                        }
                        Key::Backspace => {
                            cursor_pos = (cursor_pos.0 - 1, cursor_pos.1);

                            if cursor_pos.0 != 0 {
                                if (cursor_pos.0 - 1) as usize == self.username_password[self.field_idx].len() {
                                    self.username_password[self.field_idx].pop();
                                } else {
                                    self.username_password[self.field_idx].remove((cursor_pos.0 - 1) as usize);
                                }
                            }

                            if self.field_idx == 1 {
                                self.password_stars.pop();

                                terminal.draw(|f| self.draw(f, Some(cursor_pos)))?;

                                return Ok(());
                            }

                            terminal.draw(|f| self.draw(f, Some(cursor_pos)))?;

                            Ok(())
                        }
                        Key::Char(c) => {
                            cursor_pos = (cursor_pos.0 + 1, cursor_pos.1);

                            self.username_password[self.field_idx].insert((cursor_pos.0 - 2) as usize, c);

                            if self.field_idx == 1 {
                                self.password_stars.push('*');

                                terminal.draw(|f| self.draw(f, Some(cursor_pos)))?;

                                return Ok(());
                            }

                            terminal.draw(|f| self.draw(f, Some(cursor_pos)))?;

                            Ok(())
                        }

                        Key::Left => {
                            cursor_pos = (cursor_pos.0 - 1, cursor_pos.1);

                            terminal.set_cursor(cursor_pos.0 , cursor_pos.1)?;

                            terminal.draw(|f| self.draw(f, Some(cursor_pos)))?;

                            Ok(())
                        }

                        Key::Right => {
                            cursor_pos = (cursor_pos.0 + 1, cursor_pos.1);

                            terminal.set_cursor(cursor_pos.0, cursor_pos.1)?;

                            terminal.draw(|f| self.draw(f, Some(cursor_pos)))?;

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
        terminal.draw(|f| self.draw(f, None))?;

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
