use super::page::{InputMode, LoginPage};
use crate::event::{Event, Key};
use std::time::Duration;
use tui::{backend::Backend, Terminal};

use unicode_segmentation::UnicodeSegmentation;

use crossterm::event::{KeyEvent, MouseEvent, MouseEventKind, MouseButton};

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

                            match self.field_idx {
                                0 => {
                                    cursor_pos = (cursor_pos.0, cursor_pos.1 - 3);
                                }
                                1 => {
                                    cursor_pos = (cursor_pos.0, cursor_pos.1 + 3);
                                }

                                _ => {}
                            }

                            terminal.draw(|f| self.draw(f, Some(cursor_pos)))?;

                            Ok(())
                        }
                        Key::Backspace => {
                            cursor_pos = (cursor_pos.0 - 1, cursor_pos.1);

                            if cursor_pos.0 != 0 {
                                if (cursor_pos.0 - 1) as usize
                                    == self.username_password[self.field_idx].chars().count()
                                {
                                    self.username_password[self.field_idx].pop();
                                } else {
                                    let mut str_chars = UnicodeSegmentation::graphemes(
                                        self.username_password[self.field_idx].as_str(),
                                        true,
                                    )
                                    .map(|c| c.to_string())
                                    .collect::<Vec<String>>();

                                    str_chars.remove((cursor_pos.0 - 1) as usize);

                                    self.username_password[self.field_idx] = String::new();
                                    for str in str_chars {
                                        self.username_password[self.field_idx]
                                            .push_str(str.as_str());
                                    }
                                }
                            }

                            if self.field_idx == 1 {
                                self.password_stars.pop();
                            }

                            terminal.draw(|f| self.draw(f, Some(cursor_pos)))?;

                            Ok(())
                        }
                        Key::Char(c) => {
                            cursor_pos = (cursor_pos.0 + 1, cursor_pos.1);

                            let mut str_chars = UnicodeSegmentation::graphemes(
                                self.username_password[self.field_idx].as_str(),
                                true,
                            )
                            .map(|c| c.to_string())
                            .collect::<Vec<String>>();

                            str_chars.insert((cursor_pos.0 - 2) as usize, c.to_string());
                            self.username_password[self.field_idx] = String::new();
                            for str in str_chars {
                                self.username_password[self.field_idx].push_str(str.as_str());
                            }

                            if self.field_idx == 1 {
                                self.password_stars.push('*');
                            }

                            terminal.draw(|f| self.draw(f, Some(cursor_pos)))?;

                            Ok(())
                        }

                        Key::Left => {
                            cursor_pos = (cursor_pos.0 - 1, cursor_pos.1);

                            terminal.set_cursor(cursor_pos.0, cursor_pos.1)?;

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
            crossterm::event::Event::Mouse(cm) => match cm.kind {
                MouseEventKind::Down(cmd) => match cmd {
                    MouseButton::Left => {
                        cursor_pos = (cm.column, cm.row);

                        terminal.draw(|f| self.draw(f, Some(cursor_pos)))?;

                        Ok(())
                    },
                    MouseButton::Middle => todo!(),

                    _ => Ok(())
                },
                
                _ => Ok(())
            },
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
