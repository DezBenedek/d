mod common;
mod music;
mod youtube;

use crate::cli::DownloadCommands;

pub fn run(command: DownloadCommands) {
    match command {
        DownloadCommands::Youtube { url, quality } => youtube::run(url, quality),
        DownloadCommands::Music { url, quality } => music::run(url, quality),
    }
}
