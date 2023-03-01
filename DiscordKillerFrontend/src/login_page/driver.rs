use super::page::{LoginPage, SelectedWidget, NextPage};
use crate::{
    event::{Event, Key},
    UserFromServer,
};
use crossterm::event::KeyEvent;
use std::time::Duration;
use tui::{backend::Backend, Terminal};

impl<'a> LoginPage<'a> {
    fn input_key(&mut self, key: KeyEvent) {
        match self.selected_widget {
            SelectedWidget::UsernameInput => {
                self.username_input.input(tui_textarea::Input::from(key));
            }
            SelectedWidget::PasswordInput => {
                self.password_input.input(tui_textarea::Input::from(key));
            }
            _ => {}
        }
        self.should_redraw = true;
    }

    fn handle_ce(&mut self, ce: crossterm::event::Event) {
        match ce {
            crossterm::event::Event::FocusGained => {}
            crossterm::event::Event::FocusLost => {}
            crossterm::event::Event::Key(ck) => match Key::from(ck) {

                Key::Enter => {
                    self.add_selected_widget_counter();

                    match self.selected_widget {
                        SelectedWidget::UsernameInput => {
                            self.selected_widget = SelectedWidget::PasswordInput;
                            self.should_redraw = true;
                        }
                        SelectedWidget::PasswordInput => {
                            self.selected_widget = SelectedWidget::LoginButton;
                            self.should_redraw = true;
                        }
                        SelectedWidget::LoginButton => self.should_submit = true,
                        SelectedWidget::RegisterLink => self.should_register = true,
                    }
                },
                Key::Tab => {
                    self.add_selected_widget_counter();
                    self.selected_widget = SelectedWidget::from_i32(self.selected_widget_counter);
                    self.should_redraw = true;
                },
                Key::ShiftTab => {
                    self.subtract_selected_widget_counter();
                    self.selected_widget = SelectedWidget::from_i32(self.selected_widget_counter);
                    self.should_redraw = true;
                },
                Key::Ctrl('q') => {
                    self.should_quit = true;
                }
                Key::Ctrl('h') => {
                    self.hide_password = !self.hide_password;
                    self.should_redraw = true;
                }
                _ => self.input_key(ck),
            },
            crossterm::event::Event::Mouse(_) => {}
            crossterm::event::Event::Paste(_) => {}
            crossterm::event::Event::Resize(_, _) => {
                self.should_redraw = true;
            }
        }
    }

    fn on_tick(&mut self) {
        //self.should_redraw = true;
    }

    pub async fn run_page<B: Backend>(
        mut self,
        terminal: &mut Terminal<B>,
        tick_rate: Duration,
    ) -> std::io::Result<NextPage> {
        terminal.draw(|f| self.draw(f))?;

        loop {
            let curr_event = self.event_manager.get(tick_rate).await;

            match curr_event {
                Event::CrosstermEvent(ce) => self.handle_ce(ce),
                Event::Tick => self.on_tick(),
            }

            if self.should_redraw {
                self.error = false;
                terminal.draw(|f| self.draw(f))?;
                self.should_redraw = false;
            }

            if self.should_quit {
                return Ok(NextPage::Quit);
            } else if self.should_submit {
                use sha256::digest;
                let username = self.username_input.lines()[0].clone().to_string();
                let password = digest(self.password_input.lines()[0].clone().to_string());

                if username == "" || password == "" {
                    self.error = true;
                    self.error_message = "User was not provided";
                    terminal.draw(|f| self.draw(f))?;
                    self.should_submit = false;
                    continue;
                }

                let resp = self
                    .http_client
                    .get(format!(
                        "http://127.0.0.1:8080/login/{}:{}",
                        username, password
                    ))
                    .send()
                    .await
                    .unwrap();

                let resp_bytes = resp.bytes().await.unwrap().to_vec();

                if resp_bytes == b"User does not exist" {
                    self.error = true;
                    self.error_message = "User does not exist";
                    terminal.draw(|f| self.draw(f))?;
                    self.should_submit = false;
                } else if resp_bytes == b"Wrong password" {
                    self.error = true;
                    self.error_message = "Wrong password";
                    terminal.draw(|f| self.draw(f))?;
                    self.should_submit = false;
                } else {
                    let resp_str = String::from_utf8_lossy(&resp_bytes);
                    let user: UserFromServer = serde_json::from_str(&resp_str)?;

                    return Ok(NextPage::Main(user));
                }
                self.should_submit = false;
            } else if self.should_register {
                return Ok(NextPage::Register);
            }
        }
    }
}
