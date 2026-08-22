use crate::i18n::{
    CANCELLED, ENTER_YN, ERR_COMMAND_FAILED, ERR_START_PROGRAM, ERR_STDIN_READ, ERR_STDOUT_FLUSH,
    FIELD_REQUIRED, YT_BREW_MISSING, YT_DISCLAIMER, YT_DONE, YT_DOWNLOADING, YT_FAIL,
    YT_FOLDER_PATH_PROMPT, YT_FOLDER_PROMPT, YT_INSTALL, YT_INSTALLING, YT_INVALID_URL,
    YT_NOT_A_DIR, YT_PATH, YT_REQUIRED, YT_TOOL_INSTALLED, YT_TOOL_MISSING, YT_TOOL_OK,
    YT_URL_PROMPT, is_no, is_yes, tr, trf,
};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub fn run(url: Option<String>) {
    if let Err(error) = download(url) {
        eprintln!("{}", trf(&YT_FAIL, &[("error", &error)]));
        std::process::exit(1);
    }
}

fn download(url: Option<String>) -> Result<(), String> {
    println!("{}", tr(&YT_DISCLAIMER));
    println!();

    ensure_tool("yt-dlp", &["--version"])?;
    ensure_tool("ffmpeg", &["-version"])?;

    let url = match url {
        Some(value) if !value.trim().is_empty() => value.trim().to_string(),
        _ => prompt_required(tr(&YT_URL_PROMPT))?,
    };

    if !is_youtube_url(&url) {
        return Err(tr(&YT_INVALID_URL).to_string());
    }

    let folder = match choose_folder()? {
        Some(path) => path,
        None => {
            println!("{}", tr(&CANCELLED));
            return Ok(());
        }
    };

    println!("{}", trf(&YT_DOWNLOADING, &[("path", &folder)]));
    run_command_inherit(
        "yt-dlp",
        &[
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

fn ensure_tool(program: &str, version_args: &[&str]) -> Result<(), String> {
    if command_succeeds(program, version_args) {
        println!("{}", trf(&YT_TOOL_OK, &[("tool", program)]));
        return Ok(());
    }

    println!("{}", trf(&YT_TOOL_MISSING, &[("tool", program)]));
    if !command_succeeds("brew", &["--version"]) {
        return Err(tr(&YT_BREW_MISSING).to_string());
    }

    let label = trf(&YT_INSTALL, &[("tool", program)]);
    if !prompt_yes_no(&label, true)? {
        return Err(trf(&YT_REQUIRED, &[("tool", program)]));
    }

    println!("{}", trf(&YT_INSTALLING, &[("tool", program)]));
    run_command("brew", &["install", program])?;
    if !command_succeeds(program, version_args) {
        return Err(trf(&YT_PATH, &[("tool", program)]));
    }
    println!("{}", trf(&YT_TOOL_INSTALLED, &[("tool", program)]));
    Ok(())
}

fn choose_folder() -> Result<Option<String>, String> {
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

fn expand_path(raw: &str) -> PathBuf {
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

fn escape_applescript_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn is_user_canceled(stderr: &str) -> bool {
    stderr.contains("(-128)") || stderr.to_ascii_lowercase().contains("user canceled")
}

fn is_youtube_url(raw: &str) -> bool {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return false;
    }

    let without_scheme = trimmed
        .strip_prefix("https://")
        .or_else(|| trimmed.strip_prefix("http://"))
        .unwrap_or(trimmed);

    let host = without_scheme
        .split(['/', '?', '#'])
        .next()
        .unwrap_or("")
        .trim()
        .trim_end_matches(':')
        .to_ascii_lowercase();
    let host = host.strip_prefix("www.").unwrap_or(&host);

    matches!(
        host,
        "youtube.com" | "m.youtube.com" | "music.youtube.com" | "youtu.be" | "youtube-nocookie.com"
    )
}

fn prompt_required(label: &str) -> Result<String, String> {
    loop {
        let value = prompt(&format!("{label}: "))?;
        if !value.is_empty() {
            return Ok(value);
        }
        eprintln!("{}", tr(&FIELD_REQUIRED));
    }
}

fn prompt_yes_no(label: &str, default_yes: bool) -> Result<bool, String> {
    let hint = if default_yes { "Y/n" } else { "y/N" };
    loop {
        let answer = prompt(&format!("{label} [{hint}]: "))?;
        if answer.is_empty() {
            return Ok(default_yes);
        }
        if is_yes(&answer) {
            return Ok(true);
        }
        if is_no(&answer) {
            return Ok(false);
        }
        eprintln!("{}", tr(&ENTER_YN));
    }
}

fn prompt(message: &str) -> Result<String, String> {
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

fn run_command(program: &str, args: &[&str]) -> Result<(), String> {
    let status = Command::new(program).args(args).status().map_err(|error| {
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

fn run_command_inherit(program: &str, args: &[&str]) -> Result<(), String> {
    let status = Command::new(program)
        .args(args)
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

fn command_succeeds(program: &str, args: &[&str]) -> bool {
    Command::new(program)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
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
        assert!(!is_youtube_url(""));
        assert!(!is_youtube_url("not a url"));
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
