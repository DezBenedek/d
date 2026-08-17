use crate::i18n::{
    ERR_EXIT_CODE, ERR_START_PROGRAM, GIT_FIX_DONE, GIT_FIX_EMPTY, GIT_FIX_LIST_ERR,
    GIT_FIX_REMOVING, GIT_FIX_UNTRACK_ERR, tr, trf,
};
use std::process::Command;

pub fn run() {
    let ignored_tracked_files = match list_ignored_tracked_files() {
        Ok(files) => files,
        Err(error) => {
            eprintln!("{}", trf(&GIT_FIX_LIST_ERR, &[("error", &error)]));
            std::process::exit(1);
        }
    };

    if ignored_tracked_files.is_empty() {
        println!("{}", tr(&GIT_FIX_EMPTY));
        return;
    }

    println!(
        "{}",
        trf(
            &GIT_FIX_REMOVING,
            &[("count", &ignored_tracked_files.len().to_string())]
        )
    );
    for file in &ignored_tracked_files {
        println!("  {file}");
    }

    if let Err(error) = untrack_files(&ignored_tracked_files) {
        eprintln!("{}", trf(&GIT_FIX_UNTRACK_ERR, &[("error", &error)]));
        std::process::exit(1);
    }

    println!("{}", tr(&GIT_FIX_DONE));
}

fn list_ignored_tracked_files() -> Result<Vec<String>, String> {
    let output = Command::new("git")
        .args(["ls-files", "-ci", "--exclude-standard"])
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
