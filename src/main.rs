mod cli;
mod commands;

use clap::{CommandFactory, Parser};
use cli::{Cli, Commands};

const AUTHOR_NAME: &str = "Dezső Benedek";
const DOCUMENTATION_URL: &str = "https://github.com/DezBenedek/d";

fn main() {
    let cli = Cli::parse();

    if cli.authors {
        println!("{AUTHOR_NAME}");
        return;
    }

    if cli.doc {
        println!("{DOCUMENTATION_URL}");
        return;
    }

    match cli.command {
        Some(Commands::Ip) => commands::ip::run(),
        Some(Commands::Version) => {
            println!("d {}", env!("CARGO_PKG_VERSION"));
        }
        Some(Commands::Update) => commands::update::run(),
        Some(Commands::Push { message }) => commands::push::run(message),
        Some(Commands::Git { command }) => commands::git::run(command),
        Some(Commands::Macos { command }) => commands::macos::run(command),
        Some(Commands::Gen { command }) => commands::generate::run(command),
        None => {
            let mut command = Cli::command();
            command.print_help().ok();
            println!();
        }
    }
}
