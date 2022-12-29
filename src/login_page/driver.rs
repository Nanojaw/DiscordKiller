use super::page::{InputMode, LoginPage};
use crate::event::{Event, Key};
use std::{
    io::{Error, Result},
    time::Duration,
    time::Instant, ops::Deref,
};
use tui::{backend::Backend, Terminal};

impl<'a, C: Backend> LoginPage<'a, C> {

    pub fn handle_ce(&mut self, ce: crossterm::event::Event) {
        match ce {
            crossterm::event::Event::FocusGained => todo!(),
            crossterm::event::Event::FocusLost => todo!(),
            crossterm::event::Event::Key(ck) => {

                match self.input_mode {
                InputMode::Normal => match Key::from(ck) {
                    Key::Ctrl('c') => self.should_quit = true,

                    Key::Char('e') => {
                        self.input_mode = InputMode::Editing;

                        self.terminal.draw(|f| { self.draw(f) });
                    },
                    _ => {}
                },
                InputMode::Editing => match Key::from(ck) {
                    _ => {}
                },
            }
            },
            crossterm::event::Event::Mouse(_) => todo!(),
            crossterm::event::Event::Paste(_) => todo!(),
            crossterm::event::Event::Resize(_, _) => todo!(),
        }
    }

    pub fn on_tick(&self) {}

    pub async fn run_app(
        mut self,
        tick_rate: Duration,
    ) -> std::io::Result<()> {
        loop {
            let curr_event = self.event_manager.get(tick_rate).await;

            match curr_event {
                Event::CrosstermEvent(ce) => self.handle_ce(ce),
                Event::Tick => self.on_tick(),
            }

            /*
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
