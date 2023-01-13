use clap::{Parser, Subcommand};
#[allow(unused_imports)]
use std::fs;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: SubCommands,
}

#[derive(Subcommand)]
pub enum SubCommands {
    /// Initialise a new repository
    Init,
}

// Usage: your_git.sh <command> <arg1> <arg2> ...
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    // let git_cli = Cli::parse();
    // match git_cli.command {
    //     SubCommands::Init => {
    //         fs::create_dir(".git").unwrap();
    //         fs::create_dir(".git/objects").unwrap();
    //         fs::create_dir(".git/refs").unwrap();
    //         fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
    //         println!("Initialized git directory")
    //     }
    // }
}
