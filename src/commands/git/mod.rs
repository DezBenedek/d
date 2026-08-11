mod fix;
mod setup;
mod update;

use crate::cli::GitCommands;

pub fn run(command: GitCommands) {
    match command {
        GitCommands::Fix => fix::run(),
        GitCommands::Setup => setup::run(),
        GitCommands::Update => update::run(),
    }
}
