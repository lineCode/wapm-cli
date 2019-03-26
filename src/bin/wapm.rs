use structopt::StructOpt;
use wapm_cli::commands;

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "whoami")]
    /// Prints the current user (if authed) in the stdout
    WhoAmI,

    #[structopt(name = "login")]
    /// Logins into wapm, saving the token locally for future commands
    Login,

    #[structopt(name = "logout")]
    /// Remove the token for the registry
    Logout,

    #[structopt(name = "config")]
    /// Config related subcommands
    Config(commands::ConfigOpt),

    #[structopt(name = "add")]
    /// Add a package
    Add(commands::AddOpt),

    #[structopt(name = "publish")]
    /// Publish a package
    Publish,

    #[structopt(name = "search")]
    /// Search packages
    Search(commands::SearchOpt),
}

fn main() -> Result<(), failure::Error> {
    // dotenv::dotenv().ok();
    // env_logger::init();
    // let config: Env = envy::from_env()?;

    let args = Command::from_args();
    match args {
        Command::WhoAmI => commands::whoami(),
        Command::Login => commands::login(),
        Command::Logout => commands::logout(),
        Command::Config(config_options) => commands::config(config_options),
        Command::Add(add_options) => commands::add(add_options),
        Command::Publish => commands::publish(),
        Command::Search(search_options) => commands::search(search_options),
    }
}