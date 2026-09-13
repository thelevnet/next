use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "next", version = "1.1.6", about = "NixOS helper for github:thelevnet/dots")]
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
    /// sync configuration with git
    Sync {
        /// commit message
        message: Option<String>,
    },
    /// search nixpkgs
    Search {
        /// search query
        query: Option<String>,
    },
    /// development shell
    Dev,
    #[command(hide = true, name = "__search-query")]
    SearchQuery {
        query: Vec<String>,
    },
}

#[derive(Subcommand)]
pub enum OsAction {
    Switch,
    Dry,
    Clean,
}

#[derive(Subcommand)]
pub enum HomeAction {
    Switch,
}


