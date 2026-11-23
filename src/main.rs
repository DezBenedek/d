mod cli;
mod commands;
mod i18n;

use clap::{CommandFactory, FromArgMatches};
use cli::{Cli, Commands};

const AUTHOR_NAME: &str = "Dezső Benedek";
const DOCUMENTATION_URL: &str = "https://github.com/DezBenedek/d";

fn main() {
    i18n::init_from_args(std::env::args());

    let cmd = i18n::apply_translations(Cli::command());
    let matches = cmd.get_matches();
    let cli = match Cli::from_arg_matches(&matches) {
        Ok(cli) => cli,
        Err(error) => error.exit(),
    };

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
        Some(Commands::Port { port }) => commands::port::run_show(port),
        Some(Commands::Killport { port }) => commands::port::run_kill(port),
        Some(Commands::Serve { dir, port }) => commands::serve::run(dir, port),
        Some(Commands::Sys) => commands::sys::run(),
        Some(Commands::Version) => {
            println!("d {}", env!("CARGO_PKG_VERSION"));
        }
        Some(Commands::Update) => commands::update::run(),
        Some(Commands::Push { message }) => commands::push::run(message),
        Some(Commands::Git { command }) => commands::git::run(command),
        Some(Commands::Macos { command }) => commands::macos::run(command),
        Some(Commands::Gen { command }) => commands::generate::run(command),
        Some(Commands::Download { command }) => commands::download::run(command),
        Some(Commands::Install { command }) => commands::install::run(command),
        None => i18n::print_help(),
    }
}
