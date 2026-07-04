use std::process::Command;

pub fn run() {
    let ignored_tracked_files = match list_ignored_tracked_files() {
        Ok(files) => files,
        Err(error) => {
            eprintln!("Nem sikerült listázni a git-ignore-olt, de trackelt fájlokat: {error}");
            std::process::exit(1);
        }
    };

    if ignored_tracked_files.is_empty() {
        println!("Nincs olyan fájl, amit a .gitignore tiltana, de a git mégis trackelne.");
        return;
    }

    println!(
        "Eltávolítás a git trackingből ({} fájl):",
        ignored_tracked_files.len()
    );
    for file in &ignored_tracked_files {
        println!("  {file}");
    }

    if let Err(error) = untrack_files(&ignored_tracked_files) {
        eprintln!("Nem sikerült eltávolítani a fájlokat: {error}");
        std::process::exit(1);
    }

    println!(
        "Kész. A változást még commitolnod kell, pl.: d push gitignore-olt fajlok eltavolitasa"
    );
}

fn list_ignored_tracked_files() -> Result<Vec<String>, String> {
    let output = Command::new("git")
        .args(["ls-files", "-ci", "--exclude-standard"])
        .output()
        .map_err(|error| format!("nem sikerült elindítani a git-et: {error}"))?;

    if !output.status.success() {
        return Err(format!("kilépési kód: {:?}", output.status.code()));
    }

    let files = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();

    Ok(files)
}

fn untrack_files(files: &[String]) -> Result<(), String> {
    let status = Command::new("git")
        .arg("rm")
        .arg("-r")
        .arg("--cached")
        .arg("--")
        .args(files)
        .status()
        .map_err(|error| format!("nem sikerült elindítani a git-et: {error}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("kilépési kód: {:?}", status.code()))
    }
}
