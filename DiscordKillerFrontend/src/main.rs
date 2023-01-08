mod event;
mod login_page;
mod register_page;
mod styles;

use argh::FromArgs;
use std::{error::Error, io, time::Duration};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct UserFromServer {
    pub username: String,
    pub servers: Vec<String>,
    pub password: String,
}

impl UserFromServer {
    pub fn default() -> Self {
        Self {
            username: "".to_string(),
            servers: vec![],
            password: "".to_string(),
        }
    }
}

/// DiscordKiller args
#[derive(Debug, FromArgs)]
struct Cli {
    /// sets the number of ms between each tick
    #[argh(option, default = "16")]
    tick_rate: u64,

    /// wether or not to launch app directly into login page or to register
    #[argh(option, default = "true")]
    login: bool,

    /// subcommand
    #[argh(option, default = "false")]
    change_profile: bool,
}

enum Pages {
    Login,
    Register,
    CreateUser,
    Main,
}

pub async fn run(
    tick_rate: Duration,
    login: bool,
    change_profile: bool,
) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    let mut current_page = Pages::Login;

    let mut user: UserFromServer;

    // main loop
    loop {
        match current_page {
            Pages::Login => {
                let login_page = login_page::page::LoginPage::new("Hackchat");
                let next_page = login_page.run_page(&mut terminal, tick_rate).await?;

                match next_page {
                    login_page::page::NextPage::Register => current_page = Pages::Register,
                    login_page::page::NextPage::Main(response) => {
                        user = response;
                        current_page = Pages::Main
                    }
                    login_page::page::NextPage::Quit => break,
                }
            }
            Pages::Register => {
                let next_page = register_page::page::RegisterPage::new("Hackchat")
                    .run_page(&mut terminal, tick_rate)
                    .await?;
                    match next_page {
                        register_page::page::NextPage::CreateUser => current_page = Pages::CreateUser,
                        register_page::page::NextPage::Login => current_page = Pages::Login,
                        register_page::page::NextPage::Quit => break,
                    }
            }
            Pages::CreateUser => todo!(),
            Pages::Main => todo!(),
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = argh::from_env();
    let tick_rate = Duration::from_millis(cli.tick_rate);

    run(tick_rate, cli.login, cli.change_profile).await?;
    Ok(())
}
