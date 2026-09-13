use clap::CommandFactory;
use clap_complete::{generate_to, shells::*};
use std::fs;
use std::path::Path;

#[path = "src/cli.rs"]
mod cli;

fn main() {
    println!("cargo:rerun-if-changed=src/cli.rs");

    let out_dir = Path::new("completions");
    fs::create_dir_all(out_dir).expect("failed to create completions directory");

    let mut cmd = cli::Cli::command();
    generate_to(Bash, &mut cmd, "next", out_dir).expect("failed to generate bash completions");
    generate_to(Zsh, &mut cmd, "next", out_dir).expect("failed to generate zsh completions");
    generate_to(Fish, &mut cmd, "next", out_dir).expect("failed to generate fish completions");
}
