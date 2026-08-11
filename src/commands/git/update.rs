use crate::i18n::{
    tr, trf, ERR_EMPTY_BRANCH, ERR_EXIT_CODE, ERR_START_PROGRAM, GIT_PULL_BRANCH_ERR, GIT_PULL_FAIL,
    GIT_PULL_FETCHING, GIT_PULL_OK,
};
use std::process::Command;

pub fn run() {
    let branch = match current_branch() {
        Ok(branch) => branch,
        Err(error) => {
            eprintln!("{}", trf(&GIT_PULL_BRANCH_ERR, &[("error", &error)]));
            std::process::exit(1);
        }
    };

    println!(
        "{}",
        trf(&GIT_PULL_FETCHING, &[("branch", &branch)])
    );

    if let Err(error) = run_git(&["pull", "origin", &branch]) {
        eprintln!("{}", trf(&GIT_PULL_FAIL, &[("error", &error)]));
        std::process::exit(1);
    }

    println!("{}", tr(&GIT_PULL_OK));
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
