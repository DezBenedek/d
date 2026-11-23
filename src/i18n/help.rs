use clap::Command;

use super::{
    APP_ABOUT, CMD_DOWNLOAD, CMD_DOWNLOAD_AUDIO_QUALITY, CMD_DOWNLOAD_MUSIC,
    CMD_DOWNLOAD_MUSIC_URL, CMD_DOWNLOAD_VIDEO_QUALITY, CMD_DOWNLOAD_YOUTUBE,
    CMD_DOWNLOAD_YOUTUBE_URL, CMD_GEN, CMD_GEN_BASE64, CMD_GEN_BYTES, CMD_GEN_HEX,
    CMD_GEN_PASSWORD, CMD_GEN_PASSWORD_LEN, CMD_GEN_UUID, CMD_GIT, CMD_GIT_CLEAN, CMD_GIT_FIX,
    CMD_GIT_SETUP, CMD_GIT_UPDATE, CMD_INSTALL, CMD_INSTALL_YT_DLP, CMD_IP, CMD_KILLPORT,
    CMD_KILLPORT_NUMBER, CMD_MACOS, CMD_MACOS_DOCK, CMD_MACOS_FLUSHDNS, CMD_MACOS_RESET,
    CMD_MACOS_START, CMD_PORT, CMD_PORT_NUMBER, CMD_PUSH, CMD_PUSH_MSG, CMD_SERVE, CMD_SERVE_DIR,
    CMD_SERVE_PORT, CMD_SYS, CMD_UPDATE, CMD_VERSION, FLAG_AUTHORS, FLAG_DOC, FLAG_LANG, tr,
};

pub fn apply_translations(cmd: Command) -> Command {
    cmd.about(tr(&APP_ABOUT))
        .mut_arg("authors", |arg| arg.help(tr(&FLAG_AUTHORS)))
        .mut_arg("doc", |arg| arg.help(tr(&FLAG_DOC)))
        .mut_arg("lang", |arg| arg.help(tr(&FLAG_LANG)))
        .mut_subcommand("ip", |c| c.about(tr(&CMD_IP)))
        .mut_subcommand("port", |c| {
            c.about(tr(&CMD_PORT))
                .mut_arg("port", |arg| arg.help(tr(&CMD_PORT_NUMBER)))
        })
        .mut_subcommand("killport", |c| {
            c.about(tr(&CMD_KILLPORT))
                .mut_arg("port", |arg| arg.help(tr(&CMD_KILLPORT_NUMBER)))
        })
        .mut_subcommand("serve", |c| {
            c.about(tr(&CMD_SERVE))
                .mut_arg("dir", |arg| arg.help(tr(&CMD_SERVE_DIR)))
                .mut_arg("port", |arg| arg.help(tr(&CMD_SERVE_PORT)))
        })
        .mut_subcommand("sys", |c| c.about(tr(&CMD_SYS)))
        .mut_subcommand("version", |c| c.about(tr(&CMD_VERSION)))
        .mut_subcommand("update", |c| c.about(tr(&CMD_UPDATE)))
        .mut_subcommand("push", |c| {
            c.about(tr(&CMD_PUSH))
                .mut_arg("message", |arg| arg.help(tr(&CMD_PUSH_MSG)))
        })
        .mut_subcommand("git", |c| {
            c.about(tr(&CMD_GIT))
                .mut_subcommand("fix", |c| c.about(tr(&CMD_GIT_FIX)))
                .mut_subcommand("setup", |c| c.about(tr(&CMD_GIT_SETUP)))
                .mut_subcommand("update", |c| c.about(tr(&CMD_GIT_UPDATE)))
                .mut_subcommand("clean", |c| c.about(tr(&CMD_GIT_CLEAN)))
        })
        .mut_subcommand("macos", |c| {
            c.about(tr(&CMD_MACOS))
                .mut_subcommand("start", |c| c.about(tr(&CMD_MACOS_START)))
                .mut_subcommand("dock", |c| c.about(tr(&CMD_MACOS_DOCK)))
                .mut_subcommand("flushdns", |c| c.about(tr(&CMD_MACOS_FLUSHDNS)))
                .mut_subcommand("reset", |c| c.about(tr(&CMD_MACOS_RESET)))
        })
        .mut_subcommand("gen", |c| {
            c.about(tr(&CMD_GEN))
                .mut_subcommand("hex", |c| {
                    c.about(tr(&CMD_GEN_HEX))
                        .mut_arg("bytes", |arg| arg.help(tr(&CMD_GEN_BYTES)))
                })
                .mut_subcommand("base64", |c| {
                    c.about(tr(&CMD_GEN_BASE64))
                        .mut_arg("bytes", |arg| arg.help(tr(&CMD_GEN_BYTES)))
                })
                .mut_subcommand("uuid", |c| c.about(tr(&CMD_GEN_UUID)))
                .mut_subcommand("password", |c| {
                    c.about(tr(&CMD_GEN_PASSWORD))
                        .mut_arg("length", |arg| arg.help(tr(&CMD_GEN_PASSWORD_LEN)))
                })
        })
        .mut_subcommand("download", |c| {
            c.about(tr(&CMD_DOWNLOAD))
                .mut_subcommand("youtube", |c| {
                    c.about(tr(&CMD_DOWNLOAD_YOUTUBE))
                        .mut_arg("url", |arg| arg.help(tr(&CMD_DOWNLOAD_YOUTUBE_URL)))
                        .mut_arg("quality", |arg| arg.help(tr(&CMD_DOWNLOAD_VIDEO_QUALITY)))
                })
                .mut_subcommand("music", |c| {
                    c.about(tr(&CMD_DOWNLOAD_MUSIC))
                        .mut_arg("url", |arg| arg.help(tr(&CMD_DOWNLOAD_MUSIC_URL)))
                        .mut_arg("quality", |arg| arg.help(tr(&CMD_DOWNLOAD_AUDIO_QUALITY)))
                })
        })
        .mut_subcommand("install", |c| {
            c.about(tr(&CMD_INSTALL))
                .mut_subcommand("yt-dlp", |c| c.about(tr(&CMD_INSTALL_YT_DLP)))
        })
}

pub fn print_help() {
    use crate::cli::Cli;
    use clap::CommandFactory;

    let mut cmd = apply_translations(Cli::command());
    let _ = cmd.print_help();
    println!();
}
