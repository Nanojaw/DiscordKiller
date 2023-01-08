use super::page::{RegisterPage, SelectedWidget, NextPage};
use crate::{
    event::{Event, Key},
};
use crossterm::event::KeyEvent;
use std::time::Duration;
use tui::{backend::Backend, Terminal};

impl<'a> RegisterPage<'a> {
    fn input_key(&mut self, key: KeyEvent) {
        match self.selected_widget {
            SelectedWidget::EmailInput => {
                self.email_input.input(tui_textarea::Input::from(key));
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
                    SelectedWidget::EmailInput => {
                        self.selected_widget = SelectedWidget::RegisterButton;
                        self.should_redraw = true;
                    }
                    SelectedWidget::RegisterButton => self.should_submit = true,
                    SelectedWidget::LoginLink => self.should_login = true,
                },
                Key::Tab => match self.selected_widget {
                    SelectedWidget::EmailInput => {
                        self.selected_widget = SelectedWidget::RegisterButton;
                        self.should_redraw = true;
                    }
                    SelectedWidget::RegisterButton => {
                        self.selected_widget = SelectedWidget::LoginLink;
                        self.should_redraw = true;
                    }
                    SelectedWidget::LoginLink => {
                        self.selected_widget = SelectedWidget::EmailInput;
                        self.should_redraw = true;
                    }
                },
                Key::Ctrl('q') => {
                    self.should_quit = true;
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
                terminal.draw(|f| self.draw(f))?;
                self.should_redraw = false;
            }

            if self.should_quit {
                return Ok(NextPage::Quit);
            } else if self.should_submit {
                // server stuff
                self.should_submit = false;
                return Ok(NextPage::CreateUser);
            } else if self.should_login {
                return Ok(NextPage::Login);
            }
        }
    }
}
