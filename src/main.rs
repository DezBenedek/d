mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ip => commands::ip::run(),
        Commands::Update => commands::update::run(),
    }
}
