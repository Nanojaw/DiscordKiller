mod event;
mod login_page;
mod styles;

use argh::FromArgs;
use std::{error::Error, io, time::Duration};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal};

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

    // create app and run it
    if change_profile {
        let login = login_page::page::LoginPage::new("Hackchat");
        login.run_app(&mut terminal, tick_rate).await?;

        /*
        let change_profile = change_profile_page::page::changeProfilePage::new("Hackchat");
        change_profile_page.run_app(&mut terminal, tick_rate).await?;
        */
    } else {
        if login {
            let login = login_page::page::LoginPage::new("Hackchat");
            login.run_app(&mut terminal, tick_rate).await?;
        } else {
            /*
            let register = register_page::page::registerPage::new("Hackchat");
            register.run_app(&mut terminal, tick_rate).await?;
            */
        }

        /*
        let main_page = main_page::page::mainPage::new("Hackchat");
        main_page.run_app(&mut terminal, tick_rate).await?;
        */
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
