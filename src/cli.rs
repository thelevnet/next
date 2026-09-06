use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "next", version = "1.1.3", about = "I dunno")]
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
    /// add a package to packages.nix
    Install {
        /// leave empty to fuzzy search
        package: Option<String>,
    },
    /// drop a package from packages.nix
    Remove {
        /// leave empty to pick from what's installed
        package: Option<String>,
    },
    /// try a package without committing to it
    Temp {
        /// leave empty to fuzzy search
        package: Option<String>,
    },
    /// what's actually in packages.nix
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
    Switch {
        /// hostname target (defaults to desktop)
        #[arg(short = 'H', long)]
        hostname: Option<String>,
    },
    /// set it for next boot, don't touch the running system
    Boot {
        /// hostname target (defaults to desktop)
        #[arg(short = 'H', long)]
        hostname: Option<String>,
    },
    /// see what would change without applying anything
    Dry {
        /// hostname target (defaults to desktop)
        #[arg(short = 'H', long)]
        hostname: Option<String>,
    },
    /// jump straight to hosts/desktop
    Conf,
}

#[derive(Subcommand)]
pub enum HomeAction {
    /// activate the new home config
    Switch {
        /// configuration target (defaults to lev)
        #[arg(short = 'c', long)]
        configuration: Option<String>,
    },
    /// jump straight to users/lev
    Conf,
}
