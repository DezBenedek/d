use crate::i18n::{
    ERR_COMMAND_FAILED, ERR_START_PROGRAM, ERR_STDIN_READ, ERR_STDOUT_FLUSH, FIELD_REQUIRED,
    YT_FOLDER_PATH_PROMPT, YT_FOLDER_PROMPT, YT_NOT_A_DIR, tr, trf,
};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub const VIDEO_QUALITIES: &[&str] = &["270p", "480p", "720p", "1080p", "1440p"];
pub const AUDIO_QUALITIES: &[&str] = &["128k", "192k", "256k", "320k"];
pub const DEFAULT_VIDEO_QUALITY: &str = "1080p";
pub const DEFAULT_AUDIO_QUALITY: &str = "320k";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VideoQuality {
    pub height: u32,
}

impl VideoQuality {
    pub fn label(self) -> String {
        format!("{}p", self.height)
    }

    pub fn format_selector(self) -> String {
        let height = self.height;
        format!("bv*[height<={height}]+ba/b[height<={height}]")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioQuality {
    pub kbps: u32,
}

impl AudioQuality {
    pub fn label(self) -> String {
        format!("{}k", self.kbps)
    }

    pub fn yt_dlp_quality(self) -> String {
        format!("{}K", self.kbps)
    }

    pub fn spotdl_bitrate(self) -> String {
        format!("{}k", self.kbps)
    }
}

pub fn parse_video_quality(raw: &str) -> Option<VideoQuality> {
    let normalized = raw.trim().to_ascii_lowercase().replace('p', "");
    let height = match normalized.as_str() {
        "270" => 270,
        "480" => 480,
        "720" => 720,
        "1080" => 1080,
        "1440" => 1440,
        _ => return None,
    };
    Some(VideoQuality { height })
}

pub fn parse_audio_quality(raw: &str) -> Option<AudioQuality> {
    let normalized = raw.trim().to_ascii_lowercase().replace('k', "");
    let kbps = match normalized.as_str() {
        "128" => 128,
        "192" => 192,
        "256" => 256,
        "320" => 320,
        _ => return None,
    };
    Some(AudioQuality { kbps })
}

pub fn is_youtube_url(raw: &str) -> bool {
    matches!(
        url_host(raw).as_deref(),
        Some("youtube.com")
            | Some("m.youtube.com")
            | Some("music.youtube.com")
            | Some("youtu.be")
            | Some("youtube-nocookie.com")
    )
}

pub fn is_spotify_url(raw: &str) -> bool {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return false;
    }
    let lower = trimmed.to_ascii_lowercase();
    if lower.starts_with("spotify:") {
        return true;
    }
    matches!(
        url_host(trimmed).as_deref(),
        Some("open.spotify.com") | Some("play.spotify.com") | Some("spotify.link")
    )
}

fn url_host(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    let without_scheme = trimmed
        .strip_prefix("https://")
        .or_else(|| trimmed.strip_prefix("http://"))
        .unwrap_or(trimmed);
    let host = without_scheme
        .split(['/', '?', '#'])
        .next()?
        .trim()
        .trim_end_matches(':')
        .to_ascii_lowercase();
    if host.is_empty() || host.contains(' ') {
        return None;
    }
    Some(host.strip_prefix("www.").unwrap_or(&host).to_string())
}

pub fn choose_folder() -> Result<Option<String>, String> {
    match osascript_choose_folder(tr(&YT_FOLDER_PROMPT)) {
        Ok(Some(path)) => Ok(Some(path)),
        Ok(None) => Ok(None),
        Err(_) => prompt_folder_path(),
    }
}

fn osascript_choose_folder(prompt: &str) -> Result<Option<String>, String> {
    let escaped = escape_applescript_string(prompt);
    let script = format!("POSIX path of (choose folder with prompt \"{escaped}\")");
    let output = Command::new("osascript")
        .args(["-e", &script])
        .output()
        .map_err(|error| {
            trf(
                &ERR_START_PROGRAM,
                &[("program", "osascript"), ("error", &error.to_string())],
            )
        })?;

    if output.status.success() {
        let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if path.is_empty() {
            return Err("empty folder path".to_string());
        }
        return Ok(Some(path));
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    if is_user_canceled(&stderr) {
        return Ok(None);
    }

    Err(stderr.trim().to_string())
}

fn prompt_folder_path() -> Result<Option<String>, String> {
    let raw = prompt_required(tr(&YT_FOLDER_PATH_PROMPT))?;
    let path = expand_path(&raw);
    if !path.is_dir() {
        return Err(trf(&YT_NOT_A_DIR, &[("path", &path.display().to_string())]));
    }
    Ok(Some(path.to_string_lossy().into_owned()))
}

pub fn expand_path(raw: &str) -> PathBuf {
    let trimmed = raw.trim();
    if trimmed == "~" {
        return home_dir().unwrap_or_else(|| PathBuf::from(trimmed));
    }
    if let Some(rest) = trimmed.strip_prefix("~/") {
        if let Some(home) = home_dir() {
            return home.join(rest);
        }
    }
    PathBuf::from(trimmed)
}

fn home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}

pub fn escape_applescript_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

pub fn is_user_canceled(stderr: &str) -> bool {
    stderr.contains("(-128)") || stderr.to_ascii_lowercase().contains("user canceled")
}

pub fn prompt_required(label: &str) -> Result<String, String> {
    loop {
        let value = prompt(&format!("{label}: "))?;
        if !value.is_empty() {
            return Ok(value);
        }
        eprintln!("{}", tr(&FIELD_REQUIRED));
    }
}

pub fn prompt_with_default(label: &str, default: &str) -> Result<String, String> {
    let value = prompt(&format!("{label} [{default}]: "))?;
    if value.is_empty() {
        Ok(default.to_string())
    } else {
        Ok(value)
    }
}

pub fn prompt(message: &str) -> Result<String, String> {
    print!("{message}");
    io::stdout()
        .flush()
        .map_err(|error| trf(&ERR_STDOUT_FLUSH, &[("error", &error.to_string())]))?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|error| trf(&ERR_STDIN_READ, &[("error", &error.to_string())]))?;
    Ok(input.trim().to_string())
}

pub fn run_tool(program: &str, args: &[&str]) -> Result<(), String> {
    let status = Command::new(program)
        .args(args)
        .env("PATH", brew_augmented_path())
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|error| {
            trf(
                &ERR_START_PROGRAM,
                &[("program", program), ("error", &error.to_string())],
            )
        })?;

    if status.success() {
        Ok(())
    } else {
        Err(trf(
            &ERR_COMMAND_FAILED,
            &[
                ("program", program),
                ("args", &args.join(" ")),
                ("code", &format!("{:?}", status.code())),
            ],
        ))
    }
}

pub fn brew_augmented_path() -> String {
    let current = std::env::var("PATH").unwrap_or_default();
    let mut prefix = String::from("/opt/homebrew/bin:/usr/local/bin");
    if let Some(home) = std::env::var_os("HOME") {
        prefix.push(':');
        prefix.push_str(
            &std::path::Path::new(&home)
                .join(".local/bin")
                .to_string_lossy(),
        );
    }
    format!("{prefix}:{current}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn youtube_url_accepts_common_hosts() {
        assert!(is_youtube_url(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
        ));
        assert!(is_youtube_url("https://youtu.be/dQw4w9WgXcQ"));
        assert!(is_youtube_url(
            "https://music.youtube.com/watch?v=dQw4w9WgXcQ"
        ));
        assert!(is_youtube_url("https://m.youtube.com/watch?v=dQw4w9WgXcQ"));
        assert!(is_youtube_url("youtube.com/watch?v=dQw4w9WgXcQ"));
        assert!(!is_youtube_url("https://vimeo.com/123"));
        assert!(!is_youtube_url("https://open.spotify.com/track/abc"));
        assert!(!is_youtube_url(""));
        assert!(!is_youtube_url("not a url"));
    }

    #[test]
    fn spotify_url_accepts_open_and_uri() {
        assert!(is_spotify_url(
            "https://open.spotify.com/track/4cOdK2wGLETKBeP5fefF4x"
        ));
        assert!(is_spotify_url(
            "https://open.spotify.com/intl-hu/album/abc123"
        ));
        assert!(is_spotify_url("https://spotify.link/abcdef"));
        assert!(is_spotify_url("spotify:track:4cOdK2wGLETKBeP5fefF4x"));
        assert!(is_spotify_url("spotify:playlist:37i9dQZF1DXcBWIGoYBM5M"));
        assert!(!is_spotify_url("https://www.youtube.com/watch?v=abc"));
        assert!(!is_spotify_url(""));
    }

    #[test]
    fn video_quality_parses_with_or_without_p() {
        assert_eq!(parse_video_quality("1080p").unwrap().height, 1080);
        assert_eq!(parse_video_quality("720").unwrap().height, 720);
        assert_eq!(parse_video_quality(" 270P ").unwrap().height, 270);
        assert_eq!(parse_video_quality("1440p").unwrap().height, 1440);
        assert!(parse_video_quality("4k").is_none());
        assert!(parse_video_quality("360p").is_none());
    }

    #[test]
    fn audio_quality_parses_with_or_without_k() {
        assert_eq!(parse_audio_quality("320k").unwrap().kbps, 320);
        assert_eq!(parse_audio_quality("192").unwrap().kbps, 192);
        assert_eq!(parse_audio_quality(" 128K ").unwrap().kbps, 128);
        assert!(parse_audio_quality("96k").is_none());
        assert!(parse_audio_quality("best").is_none());
    }

    #[test]
    fn format_selector_caps_height() {
        let selector = parse_video_quality("720p").unwrap().format_selector();
        assert!(selector.contains("height<=720"));
    }

    #[test]
    fn applescript_escape_handles_quotes() {
        assert_eq!(escape_applescript_string(r#"say "hi""#), r#"say \"hi\""#);
    }

    #[test]
    fn user_canceled_detects_error_minus_128() {
        assert!(is_user_canceled("execution error: User canceled. (-128)"));
        assert!(is_user_canceled("User canceled."));
        assert!(!is_user_canceled("osascript: command not found"));
    }

    #[test]
    fn expand_tilde_uses_home_when_set() {
        let home = home_dir().expect("HOME");
        assert_eq!(expand_path("~"), home);
        assert_eq!(expand_path("~/Movies"), home.join("Movies"));
        assert_eq!(expand_path("/tmp"), PathBuf::from("/tmp"));
    }
}
