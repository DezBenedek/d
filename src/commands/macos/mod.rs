mod dock;
mod flushdns;
mod reset;
mod start;
mod util;

use crate::cli::MacosCommands;

pub fn run(command: MacosCommands) {
    match command {
        MacosCommands::Start => start::run(),
        MacosCommands::Dock => dock::run(),
        MacosCommands::Flushdns => flushdns::run(),
        MacosCommands::Reset => reset::run(),
    }
}
