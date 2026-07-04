mod fix;
mod update;

use crate::cli::GitCommands;

pub fn run(command: GitCommands) {
    match command {
        GitCommands::Fix => fix::run(),
        GitCommands::Update => update::run(),
    }
}
