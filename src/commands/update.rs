use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const GITHUB_RELEASES_LATEST_URL: &str =
    "https://api.github.com/repos/DezBenedek/d/releases/latest";
const BINARY_ASSET_NAME: &str = "d";

pub fn run() {
    if let Err(error) = update_binary() {
        eprintln!("Frissítés sikertelen: {error}");
        std::process::exit(1);
    }
}

fn update_binary() -> Result<(), String> {
    let download_url = fetch_latest_download_url()?;
    let current_exe_path = current_executable_path()?;
    let temp_download_path = current_exe_path.with_extension("update-tmp");

    println!("Legújabb verzió letöltése innen: {download_url}");
    download_file(&download_url, &temp_download_path)?;
    make_executable(&temp_download_path)?;
    replace_current_binary(&temp_download_path, &current_exe_path)?;

    println!("Sikeres frissítés.");
    Ok(())
}

fn fetch_latest_download_url() -> Result<String, String> {
    let output = Command::new("curl")
        .args(["-fsSL", GITHUB_RELEASES_LATEST_URL])
        .output()
        .map_err(|error| format!("nem sikerült elérni a GitHub API-t: {error}"))?;

    if !output.status.success() {
        return Err(format!(
            "a GitHub API lekérdezése hibával tért vissza (kód: {:?}) - létezik már kiadás a repóban?",
            output.status.code()
        ));
    }

    let response_body = String::from_utf8_lossy(&output.stdout);
    let release: serde_json::Value = serde_json::from_str(&response_body)
        .map_err(|error| format!("nem sikerült értelmezni a GitHub válaszát: {error}"))?;

    let assets = release["assets"]
        .as_array()
        .ok_or_else(|| "a kiadásnak nincsenek csatolt fájljai (assets)".to_string())?;

    let matching_asset = assets
        .iter()
        .find(|asset| asset["name"] == BINARY_ASSET_NAME)
        .ok_or_else(|| {
            format!("nem található '{BINARY_ASSET_NAME}' nevű csatolt fájl a legújabb kiadásban")
        })?;

    matching_asset["browser_download_url"]
        .as_str()
        .map(|url| url.to_string())
        .ok_or_else(|| "a talált csatolt fájlnak nincs letöltési URL-je".to_string())
}

fn current_executable_path() -> Result<PathBuf, String> {
    env::current_exe()
        .map_err(|error| format!("nem található a saját futtatható fájl helye: {error}"))
}

fn download_file(url: &str, destination: &PathBuf) -> Result<(), String> {
    let status = Command::new("curl")
        .args(["-fsSL", "-o"])
        .arg(destination)
        .arg(url)
        .status()
        .map_err(|error| format!("nem sikerült elindítani a curl-t: {error}"))?;

    if !status.success() {
        return Err("a bináris letöltése sikertelen volt".to_string());
    }

    Ok(())
}

fn make_executable(path: &PathBuf) -> Result<(), String> {
    let status = Command::new("chmod")
        .arg("+x")
        .arg(path)
        .status()
        .map_err(|error| format!("nem sikerült futtathatóvá tenni a letöltött fájlt: {error}"))?;

    if !status.success() {
        return Err("a chmod +x sikertelen volt".to_string());
    }

    Ok(())
}

fn replace_current_binary(temp_path: &PathBuf, current_path: &PathBuf) -> Result<(), String> {
    fs::rename(temp_path, current_path).map_err(|error| {
        format!("nem sikerült lecserélni a futó binárist (jogosultság hiánya?): {error}")
    })
}
