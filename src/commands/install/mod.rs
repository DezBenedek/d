mod yt_dlp;

use crate::cli::InstallCommands;

pub fn run(command: InstallCommands) {
    match command {
        InstallCommands::YtDlp => yt_dlp::run(),
    }
}

pub(crate) use yt_dlp::{ensure_spotdl, ensure_yt_dlp_and_ffmpeg, spotdl_bin, yt_dlp_bin};
