use super::page::{LoginPage, SelectedWidget};
use crate::event::{Event, Key};
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
                Key::Enter => match self.selected_widget {
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
                },
                Key::Tab => match self.selected_widget {
                    SelectedWidget::UsernameInput => {
                        self.selected_widget = SelectedWidget::PasswordInput;
                        self.should_redraw = true;
                    }
                    SelectedWidget::PasswordInput => {
                        self.selected_widget = SelectedWidget::LoginButton;
                        self.should_redraw = true;
                    }
                    SelectedWidget::LoginButton => {
                        self.selected_widget = SelectedWidget::RegisterLink;
                        self.should_redraw = true;
                    }
                    SelectedWidget::RegisterLink => {
                        self.selected_widget = SelectedWidget::UsernameInput;
                        self.should_redraw = true;
                    }
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

    pub async fn run_app<B: Backend>(
        mut self,
        terminal: &mut Terminal<B>,
        tick_rate: Duration,
    ) -> std::io::Result<()> {
        terminal.draw(|f| self.draw(f))?;

        loop {
            let curr_event = self.event_manager.get(tick_rate).await;

            match curr_event {
                Event::CrosstermEvent(ce) => self.handle_ce(ce),
                Event::Tick => self.on_tick(),
            }

            if self.should_redraw {
                terminal.draw(|f| self.draw(f))?;
                self.should_redraw = false;
            }

            if self.should_quit {
                return Ok(());
            }
        }
    }
}
