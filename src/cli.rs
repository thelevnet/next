use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "next", version = "1.1.4", about = "NixOS helper for github:thelevnet/dots")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// system actions
    Os {
        #[command(subcommand)]
        action: OsAction,
    },
    /// home actions
    Home {
        #[command(subcommand)]
        action: HomeAction,
    },
    /// app actions
    App {
        #[command(subcommand)]
        action: AppAction,
    },
    /// git actions
    Git {
        #[command(subcommand)]
        action: GitAction,
    },
    /// development shell
    Dev,
    /// shell completions
    Completions {
        shell: clap_complete::Shell,
    },
}

#[derive(Subcommand)]
pub enum OsAction {
    Switch,
    Boot,
    Dry,
    Clean,
    Rollback,
    List,
}

#[derive(Subcommand)]
pub enum HomeAction {
    Switch,
}

#[derive(Subcommand)]
pub enum AppAction {
    Install { package: Option<String> },
    Try { package: Option<String> },
    Remove { package: Option<String> },
    List,
}

#[derive(Subcommand)]
pub enum GitAction {
    Sync,
}
