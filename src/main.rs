mod cli;
mod messages;
mod config;
mod nix_parser;
mod shell;
mod commands {
    pub mod apps;
    pub mod check;
    pub mod conf;
    pub mod gens;
    pub mod git;
    pub mod home;
    pub mod os;
    pub mod search;
}

use clap::{CommandFactory, Parser};
use clap_complete::generate;
use cli::{Cli, Commands, OsAction, HomeAction};
use std::io;
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = Cli::parse();

    let ok = match cli.command {
        Commands::Os { action } => match action {
            OsAction::Switch => commands::os::switch(),
            OsAction::Boot => commands::os::boot(),
            OsAction::Dry => commands::os::dry(),
            OsAction::Conf => commands::os::conf(),
        },
        Commands::Home { action } => match action {
            HomeAction::Switch => commands::home::switch(),
            HomeAction::Conf => commands::home::conf(),
        },
        Commands::Conf { module } => {
            commands::conf::edit(module.as_deref());
            true
        }
        Commands::Install { package } => {
            match package {
                Some(pkg) => commands::apps::install(&pkg),
                None => commands::apps::install_interactive(),
            }
        }
        Commands::Remove { package } => {
            match package {
                Some(pkg) => commands::apps::remove(&pkg),
                None => commands::apps::remove_interactive(),
            }
        }
        Commands::Temp { package } => {
            match package {
                Some(pkg) => commands::apps::temp(&pkg),
                None => commands::apps::temp_interactive(),
            }
        }

        Commands::List => { commands::apps::list(); true }
        Commands::Clean => commands::gens::clean(),
        Commands::Rollback => commands::gens::rollback(),
        Commands::Gens => commands::gens::list(),
        Commands::Sync => { commands::git::sync(); true }
        Commands::Check => commands::check::check(),
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            generate(shell, &mut cmd, name, &mut io::stdout());
            true
        }
    };

    if ok { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}
