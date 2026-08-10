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

use messages::{OS_COMMIT_MSG, HOME_COMMIT_MSG}; // Понадобится импортнуть

fn main() -> ExitCode {
    let cli = Cli::parse();
    
    let mut ok = true;
    let mut commit_msg: Option<String> = None;

    match cli.command {
        Commands::Os { action } => match action {
            OsAction::Switch => { ok = commands::os::switch(); if ok { commit_msg = Some(OS_COMMIT_MSG.to_string()); } },
            OsAction::Boot => { ok = commands::os::boot(); if ok { commit_msg = Some(OS_COMMIT_MSG.to_string()); } },
            OsAction::Dry => { ok = commands::os::dry(); },
            OsAction::Conf => { commit_msg = commands::os::conf(); },
        },
        Commands::Home { action } => match action {
            HomeAction::Switch => { ok = commands::home::switch(); if ok { commit_msg = Some(HOME_COMMIT_MSG.to_string()); } },
            HomeAction::Conf => { commit_msg = commands::home::conf(); },
        },
        Commands::Conf { module } => {
            commit_msg = commands::conf::edit(module.as_deref());
        }
        Commands::Install { package } => {
            commit_msg = match package {
                Some(pkg) => commands::apps::install(&pkg),
                None => commands::apps::install_interactive(),
            };
            ok = commit_msg.is_some();
        }
        Commands::Remove { package } => {
            commit_msg = match package {
                Some(pkg) => commands::apps::remove(&pkg),
                None => commands::apps::remove_interactive(),
            };
            ok = commit_msg.is_some();
        }
        Commands::Temp { package } => {
            ok = match package {
                Some(pkg) => commands::apps::temp(&pkg),
                None => commands::apps::temp_interactive(),
            };
        }
        Commands::List => { commands::apps::list(); }
        Commands::Clean => { ok = commands::gens::clean(); }
        Commands::Rollback => { ok = commands::gens::rollback(); }
        Commands::Gens => { ok = commands::gens::list(); }
        Commands::Sync => { commands::git::sync(); } // sync сам рулит коммитами
        Commands::Check => { ok = commands::check::check(); }
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            generate(shell, &mut cmd, name, &mut io::stdout());
        }
    };

    // ГЕНИАЛЬНЫЙ МОМЕНТ: Коммит вызывается только тут!
    if ok {
        if let Some(msg) = commit_msg {
            commands::git::commit_push(msg);
        }
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
