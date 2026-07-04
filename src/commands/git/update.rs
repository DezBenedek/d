use std::process::Command;

pub fn run() {
    let branch = match current_branch() {
        Ok(branch) => branch,
        Err(error) => {
            eprintln!("Nem sikerült lekérdezni az aktuális branch-et: {error}");
            std::process::exit(1);
        }
    };

    println!("Legfrissebb változások letöltése: origin/{branch}");

    if let Err(error) = run_git(&["pull", "origin", &branch]) {
        eprintln!("git pull sikertelen: {error}");
        std::process::exit(1);
    }

    println!("Sikeres frissítés.");
}

fn run_git(args: &[&str]) -> Result<(), String> {
    let status = Command::new("git")
        .args(args)
        .status()
        .map_err(|error| format!("nem sikerült elindítani a git-et: {error}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("kilépési kód: {:?}", status.code()))
    }
}

fn current_branch() -> Result<String, String> {
    let output = Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .map_err(|error| format!("nem sikerült elindítani a git-et: {error}"))?;

    if !output.status.success() {
        return Err(format!("kilépési kód: {:?}", output.status.code()));
    }

    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if branch.is_empty() {
        return Err("üres branch név (detached HEAD állapotban vagy?)".to_string());
    }

    Ok(branch)
}
