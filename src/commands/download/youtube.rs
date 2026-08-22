use super::common::{
    DEFAULT_VIDEO_QUALITY, VIDEO_QUALITIES, choose_folder, is_youtube_url, parse_video_quality,
    prompt_required, prompt_with_default, run_tool,
};
use crate::commands::install::{ensure_yt_dlp_and_ffmpeg, yt_dlp_bin};
use crate::i18n::{
    CANCELLED, YT_DISCLAIMER, YT_DONE, YT_DOWNLOADING, YT_FAIL, YT_INVALID_URL, YT_QUALITY_INVALID,
    YT_QUALITY_PROMPT, YT_URL_PROMPT, tr, trf,
};

pub fn run(url: Option<String>, quality: Option<String>) {
    if let Err(error) = download(url, quality) {
        eprintln!("{}", trf(&YT_FAIL, &[("error", &error)]));
        std::process::exit(1);
    }
}

fn download(url: Option<String>, quality: Option<String>) -> Result<(), String> {
    println!("{}", tr(&YT_DISCLAIMER));
    println!();

    ensure_yt_dlp_and_ffmpeg()?;

    let url = match url {
        Some(value) if !value.trim().is_empty() => value.trim().to_string(),
        _ => prompt_required(tr(&YT_URL_PROMPT))?,
    };

    if !is_youtube_url(&url) {
        return Err(tr(&YT_INVALID_URL).to_string());
    }

    let quality = resolve_video_quality(quality)?;
    let format = quality.format_selector();
    println!("{}", quality.label());

    let folder = match choose_folder()? {
        Some(path) => path,
        None => {
            println!("{}", tr(&CANCELLED));
            return Ok(());
        }
    };

    let yt_dlp = yt_dlp_bin()?;
    println!("{}", trf(&YT_DOWNLOADING, &[("path", &folder)]));
    run_tool(
        &yt_dlp,
        &[
            "-f",
            &format,
            "--paths",
            &folder,
            "--output",
            "%(title)s [%(id)s].%(ext)s",
            "--merge-output-format",
            "mp4",
            "--no-playlist",
            &url,
        ],
    )?;
    println!("{}", tr(&YT_DONE));
    Ok(())
}

fn resolve_video_quality(quality: Option<String>) -> Result<super::common::VideoQuality, String> {
    if let Some(value) = quality {
        return parse_video_quality(&value).ok_or_else(|| tr(&YT_QUALITY_INVALID).to_string());
    }

    let options = VIDEO_QUALITIES.join(", ");
    loop {
        let answer = prompt_with_default(
            &trf(&YT_QUALITY_PROMPT, &[("options", &options)]),
            DEFAULT_VIDEO_QUALITY,
        )?;
        if let Some(parsed) = parse_video_quality(&answer) {
            return Ok(parsed);
        }
        eprintln!("{}", tr(&YT_QUALITY_INVALID));
    }
}
