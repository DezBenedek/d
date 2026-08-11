use crate::i18n::{
    tr, trf, UPDATE_API_PARSE, UPDATE_API_REACH, UPDATE_API_STATUS, UPDATE_CHMOD_FAIL,
    UPDATE_CHMOD_START, UPDATE_DOWNLOADING, UPDATE_DOWNLOAD_FAIL, UPDATE_EXE_PATH, UPDATE_FAIL,
    UPDATE_NO_ASSETS, UPDATE_NO_ASSET_NAME, UPDATE_NO_URL, UPDATE_OK, UPDATE_REPLACE_FAIL,
    UPDATE_CURL_START,
};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const GITHUB_RELEASES_LATEST_URL: &str =
    "https://api.github.com/repos/DezBenedek/d/releases/latest";
const BINARY_ASSET_NAME: &str = "d";

pub fn run() {
    if let Err(error) = update_binary() {
        eprintln!("{}", trf(&UPDATE_FAIL, &[("error", &error)]));
        std::process::exit(1);
    }
}

fn update_binary() -> Result<(), String> {
    let download_url = fetch_latest_download_url()?;
    let current_exe_path = current_executable_path()?;
    let temp_download_path = current_exe_path.with_extension("update-tmp");

    println!(
        "{}",
        trf(&UPDATE_DOWNLOADING, &[("url", &download_url)])
    );
    download_file(&download_url, &temp_download_path)?;
    make_executable(&temp_download_path)?;
    replace_current_binary(&temp_download_path, &current_exe_path)?;

    println!("{}", tr(&UPDATE_OK));
    Ok(())
}

fn fetch_latest_download_url() -> Result<String, String> {
    let output = Command::new("curl")
        .args(["-fsSL", GITHUB_RELEASES_LATEST_URL])
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

    let assets = release["assets"]
        .as_array()
        .ok_or_else(|| tr(&UPDATE_NO_ASSETS).to_string())?;

    let matching_asset = assets
        .iter()
        .find(|asset| asset["name"] == BINARY_ASSET_NAME)
        .ok_or_else(|| trf(&UPDATE_NO_ASSET_NAME, &[("name", BINARY_ASSET_NAME)]))?;

    matching_asset["browser_download_url"]
        .as_str()
        .map(|url| url.to_string())
        .ok_or_else(|| tr(&UPDATE_NO_URL).to_string())
}

fn current_executable_path() -> Result<PathBuf, String> {
    env::current_exe()
        .map_err(|error| trf(&UPDATE_EXE_PATH, &[("error", &error.to_string())]))
}

fn download_file(url: &str, destination: &PathBuf) -> Result<(), String> {
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

fn make_executable(path: &PathBuf) -> Result<(), String> {
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

fn replace_current_binary(temp_path: &PathBuf, current_path: &PathBuf) -> Result<(), String> {
    fs::rename(temp_path, current_path).map_err(|error| {
        trf(
            &UPDATE_REPLACE_FAIL,
            &[("error", &error.to_string())],
        )
    })
}
