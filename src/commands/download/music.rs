use super::common::{
    AUDIO_QUALITIES, DEFAULT_AUDIO_QUALITY, choose_folder, is_spotify_url, is_youtube_url,
    parse_audio_quality, prompt_required, prompt_with_default, run_tool,
};
use crate::commands::install::{ensure_spotdl, ensure_yt_dlp_and_ffmpeg, spotdl_bin, yt_dlp_bin};
use crate::i18n::{
    CANCELLED, MUSIC_FAIL, MUSIC_INVALID_URL, MUSIC_QUALITY_INVALID, MUSIC_QUALITY_PROMPT,
    MUSIC_URL_PROMPT, YT_DISCLAIMER, YT_DONE, YT_DOWNLOADING, tr, trf,
};

pub fn run(url: Option<String>, quality: Option<String>) {
    if let Err(error) = download(url, quality) {
        eprintln!("{}", trf(&MUSIC_FAIL, &[("error", &error)]));
        std::process::exit(1);
    }
}

fn download(url: Option<String>, quality: Option<String>) -> Result<(), String> {
    println!("{}", tr(&YT_DISCLAIMER));
    println!();

    let url = match url {
        Some(value) if !value.trim().is_empty() => value.trim().to_string(),
        _ => prompt_required(tr(&MUSIC_URL_PROMPT))?,
    };

    let youtube = is_youtube_url(&url);
    let spotify = is_spotify_url(&url);
    if !youtube && !spotify {
        return Err(tr(&MUSIC_INVALID_URL).to_string());
    }

    if spotify {
        ensure_spotdl()?;
    } else {
        ensure_yt_dlp_and_ffmpeg()?;
    }

    let quality = resolve_audio_quality(quality)?;
    println!("{}", quality.label());

    let folder = match choose_folder()? {
        Some(path) => path,
        None => {
            println!("{}", tr(&CANCELLED));
            return Ok(());
        }
    };

    println!("{}", trf(&YT_DOWNLOADING, &[("path", &folder)]));
    if spotify {
        download_spotify(&url, &folder, quality)?;
    } else {
        download_youtube_audio(&url, &folder, quality)?;
    }
    println!("{}", tr(&YT_DONE));
    Ok(())
}

fn download_youtube_audio(
    url: &str,
    folder: &str,
    quality: super::common::AudioQuality,
) -> Result<(), String> {
    let yt_dlp = yt_dlp_bin()?;
    let audio_quality = quality.yt_dlp_quality();
    run_tool(
        &yt_dlp,
        &[
            "-x",
            "--audio-format",
            "mp3",
            "--audio-quality",
            &audio_quality,
            "--paths",
            folder,
            "--output",
            "%(title)s [%(id)s].%(ext)s",
            "--no-playlist",
            url,
        ],
    )
}

fn download_spotify(
    url: &str,
    folder: &str,
    quality: super::common::AudioQuality,
) -> Result<(), String> {
    let spotdl = spotdl_bin()?;
    let bitrate = quality.spotdl_bitrate();
    let output = format!(
        "{}/{{artists}} - {{title}}.{{output-ext}}",
        folder.trim_end_matches('/')
    );
    run_tool(
        &spotdl,
        &[
            "download",
            url,
            "--bitrate",
            &bitrate,
            "--format",
            "mp3",
            "--output",
            &output,
        ],
    )
}

fn resolve_audio_quality(quality: Option<String>) -> Result<super::common::AudioQuality, String> {
    if let Some(value) = quality {
        return parse_audio_quality(&value).ok_or_else(|| tr(&MUSIC_QUALITY_INVALID).to_string());
    }

    let options = AUDIO_QUALITIES.join(", ");
    loop {
        let answer = prompt_with_default(
            &trf(&MUSIC_QUALITY_PROMPT, &[("options", &options)]),
            DEFAULT_AUDIO_QUALITY,
        )?;
        if let Some(parsed) = parse_audio_quality(&answer) {
            return Ok(parsed);
        }
        eprintln!("{}", tr(&MUSIC_QUALITY_INVALID));
    }
}
