mod youtube;

use crate::cli::DownloadCommands;

pub fn run(command: DownloadCommands) {
    match command {
        DownloadCommands::Youtube { url } => youtube::run(url),
    }
}
