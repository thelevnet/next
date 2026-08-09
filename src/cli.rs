use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "next", version = "1.1.0", about = "I dunno")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// rebuild the system
    Os {
        #[command(subcommand)]
        action: OsAction,
    },
    /// rebuild just the user config
    Home {
        #[command(subcommand)]
        action: HomeAction,
    },
    /// open a dots module, offers to rebuild after
    Conf {
        /// leave empty to browse
        module: Option<String>,
    },
    /// add a package to apps.nix
    Install {
        /// leave empty to fuzzy search
        package: Option<String>,
    },
    /// drop a package from apps.nix
    Remove {
        /// leave empty to pick from what's installed
        package: Option<String>,
    },
    /// try a package without committing to it
    Temp {
        /// leave empty to fuzzy search
        package: Option<String>,
    },
    /// what's actually in apps.nix
    List,
    /// nuke old generations
    Clean,
    /// undo the last switch
    Rollback,
    /// every generation still hanging around
    Gens,
    /// commit whatever's dirty and push
    Sync,
    /// make sure the flake isn't about to blow up
    Check,
    /// spit out a completion script for your shell
    Completions {
        shell: clap_complete::Shell,
    },
}

#[derive(Subcommand)]
pub enum OsAction {
    /// activate the new config
    Switch,
    /// set it for next boot, don't touch the running system
    Boot,
    /// see what would change without applying anything
    Dry,
    /// jump straight to configuration.nix
    Conf,
}

#[derive(Subcommand)]
pub enum HomeAction {
    /// activate the new home config
    Switch,
    /// jump straight to home.nix
    Conf,
}
