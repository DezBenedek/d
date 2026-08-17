use crate::i18n::{
    UPDATE_API_PARSE, UPDATE_API_REACH, UPDATE_API_STATUS, UPDATE_CHECKSUM_FORMAT,
    UPDATE_CHECKSUM_MISMATCH, UPDATE_CHECKSUM_MISSING, UPDATE_CHECKSUM_PARSE, UPDATE_CHMOD_FAIL,
    UPDATE_CHMOD_START, UPDATE_CURL_START, UPDATE_DOWNLOAD_FAIL, UPDATE_DOWNLOADING,
    UPDATE_EXE_PATH, UPDATE_FAIL, UPDATE_NEW_VERSION, UPDATE_NO_ASSET_NAME, UPDATE_NO_ASSETS,
    UPDATE_NO_TAG, UPDATE_NO_URL, UPDATE_OK, UPDATE_REPLACE_FAIL, UPDATE_SHA256_FAIL,
    UPDATE_SHA256_START, UPDATE_SUDO, UPDATE_SUDO_FAIL, UPDATE_UP_TO_DATE, tr, trf,
};
use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::Command;

const GITHUB_RELEASES_LATEST_URL: &str =
    "https://api.github.com/repos/DezBenedek/d/releases/latest";
const BINARY_ASSET_NAME: &str = "d";
const CHECKSUM_ASSET_NAMES: &[&str] = &["d.sha256", "SHA256SUMS", "checksums.txt"];

struct LatestRelease {
    version: String,
    binary_url: String,
    checksum_url: Option<String>,
}

pub fn run() {
    if let Err(error) = update_binary() {
        eprintln!("{}", trf(&UPDATE_FAIL, &[("error", &error)]));
        std::process::exit(1);
    }
}

fn update_binary() -> Result<(), String> {
    let release = fetch_latest_release()?;
    let current = env!("CARGO_PKG_VERSION");

    if !is_remote_newer(&release.version, current) {
        println!("{}", trf(&UPDATE_UP_TO_DATE, &[("version", current)]));
        return Ok(());
    }

    println!(
        "{}",
        trf(
            &UPDATE_NEW_VERSION,
            &[("current", current), ("latest", &release.version)]
        )
    );

    let checksum_url = release
        .checksum_url
        .ok_or_else(|| tr(&UPDATE_CHECKSUM_MISSING).to_string())?;
    let current_exe_path = current_executable_path()?;
    let temp_download_path = current_exe_path.with_extension("update-tmp");
    let checksum_path = current_exe_path.with_extension("update-sha256");

    let result: Result<(), String> = (|| {
        println!(
            "{}",
            trf(&UPDATE_DOWNLOADING, &[("url", &release.binary_url)])
        );
        download_file(&release.binary_url, &temp_download_path)?;
        download_file(&checksum_url, &checksum_path)?;
        verify_checksum(&temp_download_path, &checksum_path)?;
        make_executable(&temp_download_path)?;
        replace_current_binary(&temp_download_path, &current_exe_path)?;
        Ok(())
    })();

    let _ = fs::remove_file(&checksum_path);
    if result.is_err() {
        let _ = fs::remove_file(&temp_download_path);
    }

    result?;
    println!("{}", tr(&UPDATE_OK));
    Ok(())
}

fn fetch_latest_release() -> Result<LatestRelease, String> {
    let output = Command::new("curl")
        .args([
            "-fsSL",
            "-H",
            "Accept: application/vnd.github+json",
            "-H",
            "User-Agent: d-cli",
            GITHUB_RELEASES_LATEST_URL,
        ])
        .output()
        .map_err(|error| trf(&UPDATE_API_REACH, &[("error", &error.to_string())]))?;

    if !output.status.success() {
        return Err(trf(
            &UPDATE_API_STATUS,
            &[("code", &format!("{:?}", output.status.code()))],
        ));
    }

    let response_body = String::from_utf8_lossy(&output.stdout);
    let release: serde_json::Value = serde_json::from_str(&response_body)
        .map_err(|error| trf(&UPDATE_API_PARSE, &[("error", &error.to_string())]))?;

    let tag = release["tag_name"]
        .as_str()
        .map(str::trim)
        .filter(|tag| !tag.is_empty())
        .ok_or_else(|| tr(&UPDATE_NO_TAG).to_string())?;
    let version = tag.trim_start_matches('v').to_string();

    let assets = release["assets"]
        .as_array()
        .ok_or_else(|| tr(&UPDATE_NO_ASSETS).to_string())?;

    let matching_asset = assets
        .iter()
        .find(|asset| asset["name"] == BINARY_ASSET_NAME)
        .ok_or_else(|| trf(&UPDATE_NO_ASSET_NAME, &[("name", BINARY_ASSET_NAME)]))?;

    let binary_url = matching_asset["browser_download_url"]
        .as_str()
        .map(|url| url.to_string())
        .ok_or_else(|| tr(&UPDATE_NO_URL).to_string())?;

    let checksum_url = CHECKSUM_ASSET_NAMES.iter().find_map(|name| {
        assets.iter().find_map(|asset| {
            (asset["name"] == *name)
                .then(|| asset["browser_download_url"].as_str().map(str::to_string))
                .flatten()
        })
    });

    Ok(LatestRelease {
        version,
        binary_url,
        checksum_url,
    })
}

fn current_executable_path() -> Result<PathBuf, String> {
    env::current_exe().map_err(|error| trf(&UPDATE_EXE_PATH, &[("error", &error.to_string())]))
}

fn download_file(url: &str, destination: &Path) -> Result<(), String> {
    let status = Command::new("curl")
        .args(["-fsSL", "-o"])
        .arg(destination)
        .arg(url)
        .status()
        .map_err(|error| trf(&UPDATE_CURL_START, &[("error", &error.to_string())]))?;

    if !status.success() {
        return Err(tr(&UPDATE_DOWNLOAD_FAIL).to_string());
    }

    Ok(())
}

fn verify_checksum(binary_path: &Path, checksum_path: &Path) -> Result<(), String> {
    let contents = fs::read_to_string(checksum_path)
        .map_err(|error| trf(&UPDATE_CHECKSUM_PARSE, &[("error", &error.to_string())]))?;
    let expected = parse_sha256(&contents, BINARY_ASSET_NAME)
        .ok_or_else(|| trf(&UPDATE_CHECKSUM_FORMAT, &[("name", BINARY_ASSET_NAME)]))?;
    let actual = sha256_file(binary_path)?;

    if !actual.eq_ignore_ascii_case(&expected) {
        return Err(trf(
            &UPDATE_CHECKSUM_MISMATCH,
            &[("expected", &expected), ("actual", &actual)],
        ));
    }

    Ok(())
}

fn sha256_file(path: &Path) -> Result<String, String> {
    let output = Command::new("shasum")
        .args(["-a", "256"])
        .arg(path)
        .output()
        .map_err(|error| trf(&UPDATE_SHA256_START, &[("error", &error.to_string())]))?;

    if !output.status.success() {
        return Err(tr(&UPDATE_SHA256_FAIL).to_string());
    }

    String::from_utf8_lossy(&output.stdout)
        .split_whitespace()
        .next()
        .filter(|hash| is_sha256_hex(hash))
        .map(|hash| hash.to_ascii_lowercase())
        .ok_or_else(|| tr(&UPDATE_SHA256_FAIL).to_string())
}

fn make_executable(path: &Path) -> Result<(), String> {
    let status = Command::new("chmod")
        .arg("+x")
        .arg(path)
        .status()
        .map_err(|error| trf(&UPDATE_CHMOD_START, &[("error", &error.to_string())]))?;

    if !status.success() {
        return Err(tr(&UPDATE_CHMOD_FAIL).to_string());
    }

    Ok(())
}

fn replace_current_binary(temp_path: &Path, current_path: &Path) -> Result<(), String> {
    match fs::rename(temp_path, current_path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == ErrorKind::PermissionDenied => {
            println!("{}", tr(&UPDATE_SUDO));
            let status = Command::new("sudo")
                .arg("mv")
                .arg(temp_path)
                .arg(current_path)
                .status()
                .map_err(|error| trf(&UPDATE_SUDO_FAIL, &[("error", &error.to_string())]))?;

            if status.success() {
                Ok(())
            } else {
                Err(trf(
                    &UPDATE_SUDO_FAIL,
                    &[("error", &format!("{:?}", status.code()))],
                ))
            }
        }
        Err(error) => Err(trf(&UPDATE_REPLACE_FAIL, &[("error", &error.to_string())])),
    }
}

pub(crate) fn parse_semver(raw: &str) -> Option<(u64, u64, u64)> {
    let trimmed = raw.trim().trim_start_matches('v');
    if trimmed.is_empty() {
        return None;
    }

    let mut parts = trimmed.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next().unwrap_or("0").parse().ok()?;
    let patch = parts.next().unwrap_or("0").parse().ok()?;
    Some((major, minor, patch))
}

pub(crate) fn is_remote_newer(remote: &str, current: &str) -> bool {
    match (parse_semver(remote), parse_semver(current)) {
        (Some(remote), Some(current)) => remote > current,
        _ => true,
    }
}

pub(crate) fn parse_sha256(contents: &str, filename: &str) -> Option<String> {
    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut parts = line.split_whitespace();
        let hash = parts.next()?;
        if !is_sha256_hex(hash) {
            continue;
        }

        if let Some(name) = parts.next() {
            let name = name.trim_start_matches('*');
            let name = name.rsplit('/').next().unwrap_or(name);
            if name != filename {
                continue;
            }
        }

        return Some(hash.to_ascii_lowercase());
    }

    None
}

fn is_sha256_hex(value: &str) -> bool {
    value.len() == 64 && value.chars().all(|c| c.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_semver_accepts_v_prefix_and_partial() {
        assert_eq!(parse_semver("v1.2.3"), Some((1, 2, 3)));
        assert_eq!(parse_semver("1.2.3"), Some((1, 2, 3)));
        assert_eq!(parse_semver("1.2"), Some((1, 2, 0)));
        assert_eq!(parse_semver("1"), Some((1, 0, 0)));
        assert_eq!(parse_semver("  2.0.1  "), Some((2, 0, 1)));
        assert_eq!(parse_semver(""), None);
        assert_eq!(parse_semver("abc"), None);
    }

    #[test]
    fn is_remote_newer_compares_semver() {
        assert!(is_remote_newer("1.1.0", "1.0.0"));
        assert!(is_remote_newer("v1.0.1", "1.0.0"));
        assert!(!is_remote_newer("1.0.0", "1.0.0"));
        assert!(!is_remote_newer("1.0.0", "1.1.0"));
        assert!(is_remote_newer("unparseable", "1.0.0"));
    }

    #[test]
    fn parse_sha256_reads_gnu_and_bsd_formats() {
        let hash = "a".repeat(64);
        assert_eq!(
            parse_sha256(&format!("{hash}  d"), "d").as_deref(),
            Some(hash.as_str())
        );
        assert_eq!(
            parse_sha256(&format!("{hash} *d"), "d").as_deref(),
            Some(hash.as_str())
        );
        assert_eq!(
            parse_sha256(&format!("{hash}  ./d"), "d").as_deref(),
            Some(hash.as_str())
        );
        assert_eq!(parse_sha256(&hash, "d").as_deref(), Some(hash.as_str()));
        assert_eq!(parse_sha256(&format!("{hash}  other"), "d"), None);
        assert_eq!(parse_sha256("not-a-hash  d", "d"), None);
        assert_eq!(
            parse_sha256(&format!("# comment\n{hash}  d\n"), "d").as_deref(),
            Some(hash.as_str())
        );
    }
}
