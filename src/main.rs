mod cli;
mod shell;
mod commands {
    pub mod os;
    pub mod home;
    pub mod sync;
    pub mod search;
    pub mod dev;
}

use clap::Parser;
use cli::{Cli, Commands, HomeAction, OsAction};
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = Cli::parse();

    let success = match cli.command {
        Commands::Os { action } => match action {
            OsAction::Switch => commands::os::switch(),
            OsAction::Dry => commands::os::dry(),
            OsAction::Clean => commands::os::clean(),
        },
        Commands::Home { action } => match action {
            HomeAction::Switch => commands::home::switch(),
        },
        Commands::Sync { message } => commands::sync::sync(message),
        Commands::Search { query } => commands::search::run(query),
        Commands::Dev => commands::dev::enter(),
        Commands::SearchQuery { query } => {
            commands::search::query(&query);
            true
        }
    };

    if success {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
