mod cli;
mod commands;

use clap::Parser;
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
        Some(Commands::Update) => commands::update::run(),
        Some(Commands::Push { message }) => commands::push::run(message),
        Some(Commands::Git { command }) => commands::git::run(command),
        None => {
            eprintln!("Adj meg egy parancsot! Nézd meg: d --help");
            std::process::exit(1);
        }
    }
}
