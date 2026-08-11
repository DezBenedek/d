use crate::i18n::{
    tr, trf, ERR_EMPTY_BRANCH, ERR_EXIT_CODE, ERR_START_PROGRAM, PUSH_ADD_FAIL, PUSH_BRANCH_ERR,
    PUSH_COMMIT_WARN, PUSH_FAIL, PUSH_NEED_MSG,
};
use std::process::Command;

pub fn run(message_words: Vec<String>) {
    let message = message_words.join(" ");

    if message.trim().is_empty() {
        eprintln!("{}", tr(&PUSH_NEED_MSG));
        std::process::exit(1);
    }

    if let Err(error) = run_git(&["add", "-A"]) {
        eprintln!("{}", trf(&PUSH_ADD_FAIL, &[("error", &error)]));
        std::process::exit(1);
    }

    if let Err(error) = run_git(&["commit", "-m", &message]) {
        eprintln!("{}", trf(&PUSH_COMMIT_WARN, &[("error", &error)]));
    }

    let branch = match current_branch() {
        Ok(branch) => branch,
        Err(error) => {
            eprintln!("{}", trf(&PUSH_BRANCH_ERR, &[("error", &error)]));
            std::process::exit(1);
        }
    };

    if let Err(error) = run_git(&["push", "-u", "origin", &branch]) {
        eprintln!("{}", trf(&PUSH_FAIL, &[("error", &error)]));
        std::process::exit(1);
    }
}

fn run_git(args: &[&str]) -> Result<(), String> {
    let status = Command::new("git")
        .args(args)
        .status()
        .map_err(|error| {
            trf(
                &ERR_START_PROGRAM,
                &[("program", "git"), ("error", &error.to_string())],
            )
        })?;

    if status.success() {
        Ok(())
    } else {
        Err(trf(
            &ERR_EXIT_CODE,
            &[("code", &format!("{:?}", status.code()))],
        ))
    }
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
