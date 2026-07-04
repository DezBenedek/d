use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const LATEST_VERSION_URL: &str = "https://raw.githubusercontent.com/DezBenedek/d/main/latest.txt";

pub fn run() {
    if let Err(error) = update_binary() {
        eprintln!("Frissites sikertelen: {error}");
        std::process::exit(1);
    }
}

fn update_binary() -> Result<(), String> {
    let download_url = fetch_latest_download_url()?;
    let current_exe_path = current_executable_path()?;
    let temp_download_path = current_exe_path.with_extension("update-tmp");

    println!("Legujabb verzio letoltese innen: {download_url}");
    download_file(&download_url, &temp_download_path)?;
    make_executable(&temp_download_path)?;
    replace_current_binary(&temp_download_path, &current_exe_path)?;

    println!("Sikeres frissites.");
    Ok(())
}

fn fetch_latest_download_url() -> Result<String, String> {
    let output = Command::new("curl")
        .args(["-fsSL", LATEST_VERSION_URL])
        .output()
        .map_err(|error| format!("nem sikerult elerni a latest.txt-t: {error}"))?;

    if !output.status.success() {
        return Err(format!(
            "a latest.txt lekerdezese hibaval tert vissza (kod: {:?})",
            output.status.code()
        ));
    }

    let download_url = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if download_url.is_empty() {
        return Err("a latest.txt tartalma ures volt".to_string());
    }

    Ok(download_url)
}

fn current_executable_path() -> Result<PathBuf, String> {
    env::current_exe().map_err(|error| format!("nem talalhato a sajat futtathato fajl helye: {error}"))
}

fn download_file(url: &str, destination: &PathBuf) -> Result<(), String> {
    let status = Command::new("curl")
        .args(["-fsSL", "-o"])
        .arg(destination)
        .arg(url)
        .status()
        .map_err(|error| format!("nem sikerult elinditani a curl-t: {error}"))?;

    if !status.success() {
        return Err("a binaris letoltese sikertelen volt".to_string());
    }

    Ok(())
}

fn make_executable(path: &PathBuf) -> Result<(), String> {
    let status = Command::new("chmod")
        .arg("+x")
        .arg(path)
        .status()
        .map_err(|error| format!("nem sikerult futtathatova tenni a letoltott fajlt: {error}"))?;

    if !status.success() {
        return Err("a chmod +x sikertelen volt".to_string());
    }

    Ok(())
}

fn replace_current_binary(temp_path: &PathBuf, current_path: &PathBuf) -> Result<(), String> {
    fs::rename(temp_path, current_path)
        .map_err(|error| format!("nem sikerult lecserelni a futo binarist (jogosultsag hianya?): {error}"))
}
