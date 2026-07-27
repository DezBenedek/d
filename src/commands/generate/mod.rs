mod rand;

use crate::cli::GenCommands;

pub fn run(command: GenCommands) {
    match command {
        GenCommands::Hex { bytes } => rand::run(rand::Encoding::Hex, bytes),
        GenCommands::Base64 { bytes } => rand::run(rand::Encoding::Base64, bytes),
    }
}
