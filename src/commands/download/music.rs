use super::common::{
    AUDIO_QUALITIES, DEFAULT_AUDIO_QUALITY, apple_music_country, apple_music_track_id,
    choose_folder, is_apple_music_url, is_spotify_url, is_youtube_url, parse_audio_quality,
    prompt_required, prompt_with_default, run_tool,
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
    let apple = is_apple_music_url(&url);
    if !youtube && !spotify && !apple {
        return Err(tr(&MUSIC_INVALID_URL).to_string());
    }

    if spotify || apple {
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
    } else if apple {
        download_apple_music(&url, &folder, quality)?;
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
    spotdl_download(url, folder, quality)
}

/// Apple Music links carry DRM, so `d` does not rip Apple's own files —
/// same deal as Spotify. It reads the track's artist/title from Apple's
/// public catalog, then downloads the match from YouTube via `spotdl`
/// (no Apple ID or subscription needed).
fn download_apple_music(
    url: &str,
    folder: &str,
    quality: super::common::AudioQuality,
) -> Result<(), String> {
    match resolve_apple_query(url) {
        Some(query) => spotdl_download(&query, folder, quality),
        // Playlists (`pl.…`) have no numeric ID for the catalog lookup —
        // hand the URL to spotdl, which may resolve it, and surface its
        // error otherwise.
        None => spotdl_download(url, folder, quality),
    }
}

fn spotdl_download(
    query: &str,
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
            query,
            "--bitrate",
            &bitrate,
            "--format",
            "mp3",
            "--output",
            &output,
        ],
    )
}

/// `Rick Astley - Never Gonna Give You Up` for an Apple Music song URL,
/// via the public iTunes catalog (no auth). Falls back to `yt-dlp`'s
/// Apple Music metadata when the lookup has no usable result.
fn resolve_apple_query(url: &str) -> Option<String> {
    if let Some(query) = resolve_apple_query_via_itunes(url) {
        return Some(query);
    }
    resolve_apple_query_via_yt_dlp(url)
}

fn resolve_apple_query_via_itunes(url: &str) -> Option<String> {
    let id = apple_music_track_id(url)?;
    let country = apple_music_country(url);
    let api =
        format!("https://itunes.apple.com/lookup?id={id}&country={country}&entity=song&limit=1");
    let output = std::process::Command::new("curl")
        .args(["-fsSL", "--max-time", "15", &api])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).ok()?;
    let result = json.get("results")?.as_array()?.first()?;
    let artist = result.get("artistName")?.as_str()?;
    let title = result
        .get("trackName")
        .or_else(|| result.get("collectionName"))?
        .as_str()?;
    let query = format!("{} - {}", artist.trim(), title.trim().trim_matches('"'));
    if query.len() <= 3 {
        return None;
    }
    Some(query)
}

fn resolve_apple_query_via_yt_dlp(url: &str) -> Option<String> {
    let yt_dlp = yt_dlp_bin().ok()?;
    let output = std::process::Command::new(&yt_dlp)
        .args(["--no-playlist", "--print", "%(artist)s - %(title)s", url])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let query = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if query.is_empty() || query == "NA - NA" || query.len() <= 3 {
        return None;
    }
    Some(query)
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
