use std::process::Command;

pub fn run(message_words: Vec<String>) {
    let message = message_words.join(" ");

    if message.trim().is_empty() {
        eprintln!("Adj meg egy commit üzenetet! Pl: d push javítás kész");
        std::process::exit(1);
    }

    if let Err(error) = run_git(&["add", "-A"]) {
        eprintln!("git add sikertelen: {error}");
        std::process::exit(1);
    }

    if let Err(error) = run_git(&["commit", "-m", &message]) {
        eprintln!("git commit figyelmeztetes (pl. nincs mit commitolni): {error}");
    }

    let branch = match current_branch() {
        Ok(branch) => branch,
        Err(error) => {
            eprintln!("nem sikerült lekérdezni az aktuális branch-et: {error}");
            std::process::exit(1);
        }
    };

    if let Err(error) = run_git(&["push", "-u", "origin", &branch]) {
        eprintln!("git push sikertelen: {error}");
        std::process::exit(1);
    }
}

fn run_git(args: &[&str]) -> Result<(), String> {
    let status = Command::new("git")
        .args(args)
        .status()
        .map_err(|error| format!("nem sikerult elinditani a git-et: {error}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("kilepesi kod: {:?}", status.code()))
    }
}

fn current_branch() -> Result<String, String> {
    let output = Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .map_err(|error| format!("nem sikerult elinditani a git-et: {error}"))?;

    if !output.status.success() {
        return Err(format!("kilepesi kod: {:?}", output.status.code()));
    }

    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if branch.is_empty() {
        return Err("ures branch nev (detached HEAD allapotban vagy?)".to_string());
    }

    Ok(branch)
}
