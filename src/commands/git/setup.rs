use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn run() {
    if let Err(error) = setup() {
        eprintln!("Git setup sikertelen: {error}");
        std::process::exit(1);
    }
}

fn setup() -> Result<(), String> {
    ensure_gh()?;

    let repo_name = prompt_required("Repo név")?;
    let visibility = prompt_visibility()?;
    let org = prompt_optional("Organization (üres = saját felhasználó)")?;

    let current_name = git_config_get("user.name").unwrap_or_default();
    let current_email = git_config_get("user.email").unwrap_or_default();
    let user_name = prompt_with_default("Git user.name", &current_name)?;
    let user_email = prompt_with_default("Git user.email", &current_email)?;

    if user_name.is_empty() || user_email.is_empty() {
        return Err("a git user.name és user.email megadása kötelező".to_string());
    }

    ensure_gh_auth()?;

    let owner = if org.is_empty() {
        gh_username()?
    } else {
        org
    };
    let full_name = format!("{owner}/{repo_name}");

    ensure_git_repo()?;
    set_git_config("user.name", &user_name)?;
    set_git_config("user.email", &user_email)?;
    ensure_initial_commit()?;

    if remote_repo_exists(&full_name)? {
        println!("A remote repo már létezik: {full_name}");
        let overwrite = prompt_yes_no("Felülírod? A régi kód egy backup branchre kerül", true)?;
        if !overwrite {
            println!("Megszakítva.");
            return Ok(());
        }
        overwrite_existing_repo(&full_name, &visibility)?;
    } else {
        create_new_repo(&full_name, &visibility)?;
    }

    println!();
    println!("Kész. Repo: https://github.com/{full_name}");
    Ok(())
}

fn ensure_gh() -> Result<(), String> {
    if command_succeeds("gh", &["--version"]) {
        println!("GitHub CLI megvan.");
        return Ok(());
    }

    println!("A GitHub CLI (gh) nincs telepítve.");
    if !command_succeeds("brew", &["--version"]) {
        return Err(
            "a Homebrew sincs telepítve. Telepítsd a gh-t: https://cli.github.com/".to_string(),
        );
    }

    let install = prompt_yes_no("Telepítsem Homebrew-val (`brew install gh`)", true)?;
    if !install {
        return Err("gh nélkül a setup nem folytatható".to_string());
    }

    println!("gh telepítése...");
    run_command("brew", &["install", "gh"])?;
    if !command_succeeds("gh", &["--version"]) {
        return Err("a gh telepítése után sem elérhető a PATH-ban".to_string());
    }
    println!("gh telepítve.");
    Ok(())
}

fn ensure_gh_auth() -> Result<(), String> {
    if command_succeeds("gh", &["auth", "status"]) {
        return Ok(());
    }

    println!("Nincs bejelentkezve a GitHub CLI-be. Indítom a `gh auth login`-t...");
    run_command_inherit("gh", &["auth", "login"])?;
    if !command_succeeds("gh", &["auth", "status"]) {
        return Err("a gh auth login után sem vagy bejelentkezve".to_string());
    }
    Ok(())
}

fn ensure_git_repo() -> Result<(), String> {
    if command_succeeds("git", &["rev-parse", "--is-inside-work-tree"]) {
        return Ok(());
    }

    println!("Nincs git repo — `git init`...");
    run_command("git", &["init", "-b", "main"])?;
    Ok(())
}

fn ensure_initial_commit() -> Result<(), String> {
    if command_succeeds("git", &["rev-parse", "HEAD"]) {
        return Ok(());
    }

    let _ = run_command("git", &["add", "-A"]);
    let status = Command::new("git")
        .args(["commit", "-m", "Initial commit"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|error| format!("nem sikerült elindítani a git-et: {error}"))?;

    if !status.success() {
        // Üres munkafa — üres commit, hogy legyen mit pusholni
        run_command(
            "git",
            &[
                "commit",
                "--allow-empty",
                "-m",
                "Initial commit",
            ],
        )?;
    }

    Ok(())
}

fn create_new_repo(full_name: &str, visibility: &str) -> Result<(), String> {
    println!("Új {visibility} repo létrehozása: {full_name}");

    let mut args = vec![
        "repo",
        "create",
        full_name,
        "--source=.",
        "--remote=origin",
        "--push",
    ];
    if visibility == "private" {
        args.push("--private");
    } else {
        args.push("--public");
    }

    run_command("gh", &args)?;
    Ok(())
}

fn overwrite_existing_repo(full_name: &str, visibility: &str) -> Result<(), String> {
    ensure_origin(full_name)?;
    println!("Remote letöltése backuphoz...");
    // Lehet, hogy a remote üres — a fetch hibája nem feltétlen végzetes
    let _ = run_command("git", &["fetch", "origin"]);

    let default_branch = remote_default_branch(full_name).unwrap_or_else(|_| "main".to_string());
    let remote_ref = format!("origin/{default_branch}");
    let remote_has_branch = command_succeeds("git", &["rev-parse", "--verify", &remote_ref]);

    if remote_has_branch {
        let stamp = unix_timestamp();
        let backup_branch = format!("backup/pre-setup-{stamp}");
        println!("Régi kód mentése branchre: {backup_branch}");
        run_command(
            "git",
            &["branch", backup_branch.as_str(), remote_ref.as_str()],
        )?;
        run_command(
            "git",
            &["push", "-u", "origin", backup_branch.as_str()],
        )?;
        println!("Backup branch pusholva: {backup_branch}");
    } else {
        println!("Nincs meglévő remote tartalom a(z) `{default_branch}` branchen — nincs mit backupolni.");
    }

    // Visibility frissítése, ha kell
    let vis_flag = if visibility == "private" {
        "--visibility=private"
    } else {
        "--visibility=public"
    };
    let _ = run_command("gh", &["repo", "edit", full_name, vis_flag]);

    let local_branch = current_branch()?;
    println!("Helyi kód felülírása force push-sal: {local_branch} → origin/{default_branch}");

    if local_branch == default_branch {
        run_command(
            "git",
            &["push", "--force", "-u", "origin", local_branch.as_str()],
        )?;
    } else {
        // Helyi branch ≠ default: a defaultre pusholjuk a jelenlegi HEAD-et
        run_command(
            "git",
            &[
                "push",
                "--force",
                "-u",
                "origin",
                &format!("HEAD:refs/heads/{default_branch}"),
            ],
        )?;
        run_command(
            "git",
            &[
                "remote",
                "set-head",
                "origin",
                default_branch.as_str(),
            ],
        )?;
    }

    Ok(())
}

fn ensure_origin(full_name: &str) -> Result<(), String> {
    let url = format!("https://github.com/{full_name}.git");
    if command_succeeds("git", &["remote", "get-url", "origin"]) {
        run_command("git", &["remote", "set-url", "origin", &url])?;
    } else {
        run_command("git", &["remote", "add", "origin", &url])?;
    }
    Ok(())
}

fn remote_repo_exists(full_name: &str) -> Result<bool, String> {
    let status = Command::new("gh")
        .args(["repo", "view", full_name])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|error| format!("nem sikerült elindítani a gh-t: {error}"))?;
    Ok(status.success())
}

fn remote_default_branch(full_name: &str) -> Result<String, String> {
    let output = Command::new("gh")
        .args([
            "repo",
            "view",
            full_name,
            "--json",
            "defaultBranchRef",
            "--jq",
            ".defaultBranchRef.name",
        ])
        .output()
        .map_err(|error| format!("nem sikerült elindítani a gh-t: {error}"))?;

    if !output.status.success() {
        return Err("nem sikerült lekérdezni a default branchet".to_string());
    }

    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if branch.is_empty() || branch == "null" {
        Ok("main".to_string())
    } else {
        Ok(branch)
    }
}

fn gh_username() -> Result<String, String> {
    let output = Command::new("gh")
        .args(["api", "user", "--jq", ".login"])
        .output()
        .map_err(|error| format!("nem sikerült elindítani a gh-t: {error}"))?;

    if !output.status.success() {
        return Err("nem sikerült lekérdezni a GitHub felhasználónevet".to_string());
    }

    let username = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if username.is_empty() {
        return Err("üres GitHub felhasználónév".to_string());
    }
    Ok(username)
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
        return Err("üres branch név (detached HEAD?)".to_string());
    }
    Ok(branch)
}

fn set_git_config(key: &str, value: &str) -> Result<(), String> {
    run_command("git", &["config", key, value])
}

fn git_config_get(key: &str) -> Result<String, String> {
    let output = Command::new("git")
        .args(["config", "--get", key])
        .output()
        .map_err(|error| format!("nem sikerült elindítani a git-et: {error}"))?;

    if !output.status.success() {
        return Ok(String::new());
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn prompt_visibility() -> Result<String, String> {
    loop {
        let answer = prompt_with_default("Láthatóság (public/private)", "private")?;
        match answer.to_lowercase().as_str() {
            "public" | "pub" | "nyilvános" | "nyilvanos" => return Ok("public".to_string()),
            "private" | "priv" | "privát" | "privat" => return Ok("private".to_string()),
            _ => eprintln!("Írd be: public vagy private"),
        }
    }
}

fn prompt_required(label: &str) -> Result<String, String> {
    loop {
        let value = prompt(&format!("{label}: "))?;
        if !value.is_empty() {
            return Ok(value);
        }
        eprintln!("Ez a mező kötelező.");
    }
}

fn prompt_optional(label: &str) -> Result<String, String> {
    prompt(&format!("{label}: "))
}

fn prompt_with_default(label: &str, default: &str) -> Result<String, String> {
    let message = if default.is_empty() {
        format!("{label}: ")
    } else {
        format!("{label} [{default}]: ")
    };
    let value = prompt(&message)?;
    if value.is_empty() {
        Ok(default.to_string())
    } else {
        Ok(value)
    }
}

fn prompt_yes_no(label: &str, default_yes: bool) -> Result<bool, String> {
    let hint = if default_yes { "Y/n" } else { "y/N" };
    loop {
        let answer = prompt(&format!("{label} [{hint}]: "))?;
        if answer.is_empty() {
            return Ok(default_yes);
        }
        match answer.to_lowercase().as_str() {
            "y" | "yes" | "i" | "igen" => return Ok(true),
            "n" | "no" | "nem" => return Ok(false),
            _ => eprintln!("Írd be: y vagy n"),
        }
    }
}

fn prompt(message: &str) -> Result<String, String> {
    print!("{message}");
    io::stdout()
        .flush()
        .map_err(|error| format!("stdout flush hiba: {error}"))?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|error| format!("stdin olvasási hiba: {error}"))?;
    Ok(input.trim().to_string())
}

fn run_command(program: &str, args: &[&str]) -> Result<(), String> {
    let status = Command::new(program)
        .args(args)
        .status()
        .map_err(|error| format!("nem sikerült elindítani a(z) {program}-t: {error}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "{program} {} sikertelen (kód: {:?})",
            args.join(" "),
            status.code()
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
        .map_err(|error| format!("nem sikerült elindítani a(z) {program}-t: {error}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "{program} {} sikertelen (kód: {:?})",
            args.join(" "),
            status.code()
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

fn unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}
