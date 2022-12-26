use clap::Parser;
mod app;

#[derive(Parser)]
#[command(version)]

struct Cli {
    #[arg(short, value_name = "New user?")]
    new_user: bool,
}

fn main() -> Result<(), String> {
    println!("Gello, world!");

    let cli = Cli::parse();

    let mut discord_killer: app::App;
    
    discord_killer = app::App::new(cli.new_user);

    discord_killer.run()?;

    return Ok(());
}
