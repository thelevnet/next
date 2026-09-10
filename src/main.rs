mod cli;
mod config;
mod nix_parser;
mod shell;
mod commands {
    pub mod os;
    pub mod home;
    pub mod app;
    pub mod git;
    pub mod dev;
}
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use cli::{AppAction, Cli, Commands, GitAction, HomeAction, OsAction};
use std::io;
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Commands::Os { action } => match action {
            OsAction::Switch => { commands::os::switch(); },
            OsAction::Dry => { commands::os::dry(); },
            OsAction::Clean => { commands::os::clean(); },
            OsAction::Rollback => { commands::os::rollback(); },
            OsAction::List => { commands::os::list(); },
        },
        Commands::Home { action } => match action {
            HomeAction::Switch => { commands::home::switch(); },
        },
        Commands::App { action } => match action {
            AppAction::Install { package } => { commands::app::install(package.as_deref()); },
            AppAction::Remove { package } => { commands::app::remove(package.as_deref()); },
            AppAction::Try { package } => { commands::app::r#try(package.as_deref()); },
            AppAction::List => { commands::app::list(); },
        },
        Commands::Git { action } => match action {
            GitAction::Sync => { commands::git::sync(); },
        },
        Commands::Dev => { commands::dev::enter(); },
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            generate(shell, &mut cmd, name, &mut io::stdout());
        },
    };

    ExitCode::SUCCESS
}
