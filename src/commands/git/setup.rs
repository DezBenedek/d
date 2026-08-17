use crate::i18n::{
    CANCELLED, ENTER_VISIBILITY, ENTER_YN, ERR_COMMAND_FAILED, ERR_EMPTY_BRANCH, ERR_EXIT_CODE,
    ERR_START_PROGRAM, ERR_STDIN_READ, ERR_STDOUT_FLUSH, FIELD_REQUIRED, SETUP_BACKUP_BRANCH,
    SETUP_BACKUP_PUSHED, SETUP_BREW_MISSING, SETUP_CREATE_REPO, SETUP_DEFAULT_BRANCH_ERR,
    SETUP_DONE_REPO, SETUP_FAIL, SETUP_FETCH_BACKUP, SETUP_FORCE_PUSH, SETUP_GH_AUTH_FAIL,
    SETUP_GH_INSTALLED, SETUP_GH_INSTALLING, SETUP_GH_LOGIN, SETUP_GH_MISSING, SETUP_GH_OK,
    SETUP_GH_PATH, SETUP_GH_REQUIRED, SETUP_GH_USER_EMPTY, SETUP_GH_USER_ERR, SETUP_GIT_INIT,
    SETUP_INSTALL_GH, SETUP_NEED_IDENTITY, SETUP_NO_BACKUP, SETUP_ORG, SETUP_OVERWRITE,
    SETUP_REMOTE_EXISTS, SETUP_REPO_NAME, SETUP_VISIBILITY, is_no, is_private, is_public, is_yes,
    tr, trf,
};
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn run() {
    if let Err(error) = setup() {
        eprintln!("{}", trf(&SETUP_FAIL, &[("error", &error)]));
        std::process::exit(1);
    }
}

fn setup() -> Result<(), String> {
    ensure_gh()?;

    let repo_name = prompt_required(tr(&SETUP_REPO_NAME))?;
    let visibility = prompt_visibility()?;
    let org = prompt_optional(tr(&SETUP_ORG))?;

    let current_name = git_config_get("user.name").unwrap_or_default();
    let current_email = git_config_get("user.email").unwrap_or_default();
    let user_name = prompt_with_default("Git user.name", &current_name)?;
    let user_email = prompt_with_default("Git user.email", &current_email)?;

    if user_name.is_empty() || user_email.is_empty() {
        return Err(tr(&SETUP_NEED_IDENTITY).to_string());
    }

    ensure_gh_auth()?;

    let owner = if org.is_empty() { gh_username()? } else { org };
    let full_name = format!("{owner}/{repo_name}");

    ensure_git_repo()?;
    set_git_config("user.name", &user_name)?;
    set_git_config("user.email", &user_email)?;
    ensure_initial_commit()?;

    if remote_repo_exists(&full_name)? {
        println!("{}", trf(&SETUP_REMOTE_EXISTS, &[("name", &full_name)]));
        let overwrite = prompt_yes_no(tr(&SETUP_OVERWRITE), true)?;
        if !overwrite {
            println!("{}", tr(&CANCELLED));
            return Ok(());
        }
        overwrite_existing_repo(&full_name, &visibility)?;
    } else {
        create_new_repo(&full_name, &visibility)?;
    }

    println!();
    println!("{}", trf(&SETUP_DONE_REPO, &[("name", &full_name)]));
    Ok(())
}

fn ensure_gh() -> Result<(), String> {
    if command_succeeds("gh", &["--version"]) {
        println!("{}", tr(&SETUP_GH_OK));
        return Ok(());
    }

    println!("{}", tr(&SETUP_GH_MISSING));
    if !command_succeeds("brew", &["--version"]) {
        return Err(tr(&SETUP_BREW_MISSING).to_string());
    }

    let install = prompt_yes_no(tr(&SETUP_INSTALL_GH), true)?;
    if !install {
        return Err(tr(&SETUP_GH_REQUIRED).to_string());
    }

    println!("{}", tr(&SETUP_GH_INSTALLING));
    run_command("brew", &["install", "gh"])?;
    if !command_succeeds("gh", &["--version"]) {
        return Err(tr(&SETUP_GH_PATH).to_string());
    }
    println!("{}", tr(&SETUP_GH_INSTALLED));
    Ok(())
}

fn ensure_gh_auth() -> Result<(), String> {
    if command_succeeds("gh", &["auth", "status"]) {
        return Ok(());
    }

    println!("{}", tr(&SETUP_GH_LOGIN));
    run_command_inherit("gh", &["auth", "login"])?;
    if !command_succeeds("gh", &["auth", "status"]) {
        return Err(tr(&SETUP_GH_AUTH_FAIL).to_string());
    }
    Ok(())
}

fn ensure_git_repo() -> Result<(), String> {
    if command_succeeds("git", &["rev-parse", "--is-inside-work-tree"]) {
        return Ok(());
    }

    println!("{}", tr(&SETUP_GIT_INIT));
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
        .map_err(|error| {
            trf(
                &ERR_START_PROGRAM,
                &[("program", "git"), ("error", &error.to_string())],
            )
        })?;

    if !status.success() {
        run_command("git", &["commit", "--allow-empty", "-m", "Initial commit"])?;
    }

    Ok(())
}

fn create_new_repo(full_name: &str, visibility: &str) -> Result<(), String> {
    println!(
        "{}",
        trf(
            &SETUP_CREATE_REPO,
            &[("visibility", visibility), ("name", full_name)]
        )
    );

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
    println!("{}", tr(&SETUP_FETCH_BACKUP));
    let _ = run_command("git", &["fetch", "origin"]);

    let default_branch = remote_default_branch(full_name).unwrap_or_else(|_| "main".to_string());
    let remote_ref = format!("origin/{default_branch}");
    let remote_has_branch = command_succeeds("git", &["rev-parse", "--verify", &remote_ref]);

    if remote_has_branch {
        let stamp = unix_timestamp();
        let backup_branch = format!("backup/pre-setup-{stamp}");
        println!(
            "{}",
            trf(&SETUP_BACKUP_BRANCH, &[("branch", &backup_branch)])
        );
        run_command(
            "git",
            &["branch", backup_branch.as_str(), remote_ref.as_str()],
        )?;
        run_command("git", &["push", "-u", "origin", backup_branch.as_str()])?;
        println!(
            "{}",
            trf(&SETUP_BACKUP_PUSHED, &[("branch", &backup_branch)])
        );
    } else {
        println!("{}", trf(&SETUP_NO_BACKUP, &[("branch", &default_branch)]));
    }

    let vis_flag = if visibility == "private" {
        "--visibility=private"
    } else {
        "--visibility=public"
    };
    let _ = run_command("gh", &["repo", "edit", full_name, vis_flag]);

    let local_branch = current_branch()?;
    println!(
        "{}",
        trf(
            &SETUP_FORCE_PUSH,
            &[("local", &local_branch), ("remote", &default_branch)]
        )
    );

    if local_branch == default_branch {
        run_command(
            "git",
            &["push", "--force", "-u", "origin", local_branch.as_str()],
        )?;
    } else {
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
            &["remote", "set-head", "origin", default_branch.as_str()],
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
        .map_err(|error| {
            trf(
                &ERR_START_PROGRAM,
                &[("program", "gh"), ("error", &error.to_string())],
            )
        })?;
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
        .map_err(|error| {
            trf(
                &ERR_START_PROGRAM,
                &[("program", "gh"), ("error", &error.to_string())],
            )
        })?;

    if !output.status.success() {
        return Err(tr(&SETUP_DEFAULT_BRANCH_ERR).to_string());
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
        .map_err(|error| {
            trf(
                &ERR_START_PROGRAM,
                &[("program", "gh"), ("error", &error.to_string())],
            )
        })?;

    if !output.status.success() {
        return Err(tr(&SETUP_GH_USER_ERR).to_string());
    }

    let username = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if username.is_empty() {
        return Err(tr(&SETUP_GH_USER_EMPTY).to_string());
    }
    Ok(username)
}

fn current_branch() -> Result<String, String> {
    let output = Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .map_err(|error| {
            trf(
                &ERR_START_PROGRAM,
                &[("program", "git"), ("error", &error.to_string())],
            )
        })?;

    if !output.status.success() {
        return Err(trf(
            &ERR_EXIT_CODE,
            &[("code", &format!("{:?}", output.status.code()))],
        ));
    }

    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if branch.is_empty() {
        return Err(tr(&ERR_EMPTY_BRANCH).to_string());
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
        .map_err(|error| {
            trf(
                &ERR_START_PROGRAM,
                &[("program", "git"), ("error", &error.to_string())],
            )
        })?;

    if !output.status.success() {
        return Ok(String::new());
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn prompt_visibility() -> Result<String, String> {
    loop {
        let answer = prompt_with_default(tr(&SETUP_VISIBILITY), "private")?;
        if is_public(&answer) {
            return Ok("public".to_string());
        }
        if is_private(&answer) {
            return Ok("private".to_string());
        }
        eprintln!("{}", tr(&ENTER_VISIBILITY));
    }
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

fn unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}
