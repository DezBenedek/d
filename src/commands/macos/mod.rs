mod start;

use crate::cli::MacosCommands;

pub fn run(command: MacosCommands) {
    match command {
        MacosCommands::Start => start::run(),
    }
}
