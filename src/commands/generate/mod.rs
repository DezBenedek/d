mod secret;

use crate::cli::GenCommands;

pub fn run(command: GenCommands) {
    match command {
        GenCommands::Secret { bytes } => secret::run(bytes),
    }
}
