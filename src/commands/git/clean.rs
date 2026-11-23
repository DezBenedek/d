use crate::commands::git::fix::{list_ignored_tracked_files, untrack_files};
use crate::i18n::{
    CANCELLED, CLEAN_BRANCH_FAIL, CLEAN_BRANCHES, CLEAN_CONFIRM, CLEAN_DONE, CLEAN_EMPTY,
    CLEAN_NOT_A_REPO, CLEAN_PRUNE, CLEAN_WARN, ERR_STDIN_READ, GIT_FIX_LIST_ERR, GIT_FIX_REMOVING,
    GIT_FIX_UNTRACK_ERR, is_yes, tr, trf,
};
use std::io::{self, Write};
use std::process::Command;

const PROTECTED_BRANCHES: &[&str] = &["main", "master"];

pub fn run() {
    if !inside_work_tree() {
        eprintln!("{}", tr(&CLEAN_NOT_A_REPO));
        std::process::exit(1);
    }

    let ignored = match list_ignored_tracked_files() {
        Ok(files) => files,
        Err(error) => {
            eprintln!("{}", trf(&GIT_FIX_LIST_ERR, &[("error", &error)]));
            std::process::exit(1);
        }
    };
    let current = current_branch().unwrap_or_default();
    let merged = match merged_branches() {
        Ok(output) => parse_merged_branches(&output, &current),
        Err(error) => {
            eprintln!("{}", trf(&CLEAN_WARN, &[("error", &error)]));
            Vec::new()
        }
    };
    let has_origin = has_remote("origin");

    if ignored.is_empty() && merged.is_empty() && !has_origin {
        println!("{}", tr(&CLEAN_EMPTY));
        return;
    }

    if !ignored.is_empty() {
        println!(
            "{}",
            trf(&GIT_FIX_REMOVING, &[("count", &ignored.len().to_string())])
        );
        for file in &ignored {
            println!("  {file}");
        }
    }
    if !merged.is_empty() {
        println!(
            "{}",
            trf(&CLEAN_BRANCHES, &[("count", &merged.len().to_string())])
        );
        for branch in &merged {
            println!("  {branch}");
        }
    }
    if has_origin {
        println!("  {}", tr(&CLEAN_PRUNE));
    }

    if !confirm() {
        println!("{}", tr(&CANCELLED));
        return;
    }

    if !ignored.is_empty() {
        if let Err(error) = untrack_files(&ignored) {
            eprintln!("{}", trf(&GIT_FIX_UNTRACK_ERR, &[("error", &error)]));
            std::process::exit(1);
        }
    }
    for branch in &merged {
        if let Err(error) = delete_branch(branch) {
            eprintln!(
                "{}",
                trf(&CLEAN_BRANCH_FAIL, &[("branch", branch), ("error", &error)])
            );
        }
    }
    if has_origin {
        if let Err(error) = prune_origin() {
            eprintln!("{}", trf(&CLEAN_WARN, &[("error", &error)]));
        }
    }

    println!("{}", tr(&CLEAN_DONE));
}

fn inside_work_tree() -> bool {
    Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn current_branch() -> Option<String> {
    let output = Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn merged_branches() -> Result<String, String> {
    let output = Command::new("git")
        .args(["branch", "--merged", "HEAD"])
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err(format!("exit code: {:?}", output.status.code()));
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Local branches fully merged into HEAD, minus the current and protected ones.
fn parse_merged_branches(output: &str, current: &str) -> Vec<String> {
    output
        .lines()
        .map(|line| {
            line.trim()
                .strip_prefix("* ")
                .unwrap_or(line.trim())
                .trim()
                .to_string()
        })
        .filter(|branch| {
            !branch.is_empty()
                && branch != current
                && !PROTECTED_BRANCHES.contains(&branch.as_str())
        })
        .collect()
}

fn has_remote(name: &str) -> bool {
    Command::new("git")
        .args(["remote"])
        .output()
        .map(|output| {
            String::from_utf8_lossy(&output.stdout)
                .lines()
                .any(|line| line.trim() == name)
        })
        .unwrap_or(false)
}

fn delete_branch(branch: &str) -> Result<(), String> {
    let output = Command::new("git")
        .args(["branch", "-d", branch])
        .output()
        .map_err(|error| error.to_string())?;
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

fn prune_origin() -> Result<(), String> {
    let output = Command::new("git")
        .args(["remote", "prune", "origin"])
        .output()
        .map_err(|error| error.to_string())?;
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

fn confirm() -> bool {
    print!("{} [y/N]: ", tr(&CLEAN_CONFIRM));
    if io::stdout().flush().is_err() {
        return false;
    }
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => is_yes(input.trim()),
        Err(error) => {
            eprintln!("{}", trf(&ERR_STDIN_READ, &[("error", &error.to_string())]));
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merged_skips_current_and_protected() {
        let output = "* main\n  feature-a\n  master\n  feature-b\n";
        assert_eq!(
            parse_merged_branches(output, "main"),
            vec!["feature-a".to_string(), "feature-b".to_string()]
        );
    }

    #[test]
    fn merged_skips_non_main_current_branch() {
        let output = "  main\n* work\n  old-fix\n";
        assert_eq!(
            parse_merged_branches(output, "work"),
            vec!["old-fix".to_string()]
        );
    }

    #[test]
    fn merged_handles_empty_output() {
        assert!(parse_merged_branches("", "main").is_empty());
    }
}
