use crate::i18n::{
    ERR_COMMAND_FAILED, ERR_START_PROGRAM, INSTALL_FAIL, YT_BREW_MISSING, YT_INSTALLING, YT_PATH,
    YT_TOOL_INSTALLED, YT_TOOL_MISSING, YT_TOOL_OK, tr, trf,
};
use std::path::Path;
use std::process::{Command, Stdio};

const YT_DLP: &str = "yt-dlp";
const FFMPEG: &str = "ffmpeg";
const SPOTDL: &str = "spotdl";
const PIPX: &str = "pipx";

struct Pkg {
    name: &'static str,
    version_args: &'static [&'static str],
}

const YT_DLP_FFMPEG: &[Pkg] = &[
    Pkg {
        name: YT_DLP,
        version_args: &["--version"],
    },
    Pkg {
        name: FFMPEG,
        version_args: &["-version"],
    },
];

const PIPX_PKG: &[Pkg] = &[Pkg {
    name: PIPX,
    version_args: &["--version"],
}];

pub fn run() {
    if let Err(error) = ensure_packages(YT_DLP_FFMPEG) {
        eprintln!("{}", trf(&INSTALL_FAIL, &[("error", &error)]));
        std::process::exit(1);
    }
}

pub fn ensure_yt_dlp_and_ffmpeg() -> Result<(), String> {
    ensure_packages(YT_DLP_FFMPEG)
}

pub fn ensure_spotdl() -> Result<(), String> {
    ensure_yt_dlp_and_ffmpeg()?;
    if resolve_tool(SPOTDL, &["--version"]).is_some() {
        println!("{}", trf(&YT_TOOL_OK, &[("tool", SPOTDL)]));
        return Ok(());
    }

    println!("{}", trf(&YT_TOOL_MISSING, &[("tool", SPOTDL)]));
    ensure_packages(PIPX_PKG)?;
    let pipx =
        resolve_tool(PIPX, &["--version"]).ok_or_else(|| trf(&YT_PATH, &[("tool", PIPX)]))?;
    println!("{}", trf(&YT_INSTALLING, &[("tool", SPOTDL)]));
    run_command_inherit(&pipx, &["install", "spotdl"])?;
    if resolve_tool(SPOTDL, &["--version"]).is_none() {
        return Err(trf(&YT_PATH, &[("tool", SPOTDL)]));
    }
    println!("{}", trf(&YT_TOOL_INSTALLED, &[("tool", SPOTDL)]));
    Ok(())
}

pub fn yt_dlp_bin() -> Result<String, String> {
    resolve_tool(YT_DLP, &["--version"]).ok_or_else(|| trf(&YT_PATH, &[("tool", YT_DLP)]))
}

pub fn spotdl_bin() -> Result<String, String> {
    resolve_tool(SPOTDL, &["--version"]).ok_or_else(|| trf(&YT_PATH, &[("tool", SPOTDL)]))
}

fn ensure_packages(packages: &[Pkg]) -> Result<(), String> {
    let mut missing = Vec::new();

    for pkg in packages {
        if resolve_tool(pkg.name, pkg.version_args).is_some() {
            println!("{}", trf(&YT_TOOL_OK, &[("tool", pkg.name)]));
        } else {
            println!("{}", trf(&YT_TOOL_MISSING, &[("tool", pkg.name)]));
            missing.push(pkg);
        }
    }

    if missing.is_empty() {
        return Ok(());
    }

    let brew =
        resolve_tool("brew", &["--version"]).ok_or_else(|| tr(&YT_BREW_MISSING).to_string())?;

    let tools = missing
        .iter()
        .map(|pkg| pkg.name)
        .collect::<Vec<_>>()
        .join(", ");
    println!("{}", trf(&YT_INSTALLING, &[("tool", &tools)]));
    let mut args = vec!["install"];
    args.extend(missing.iter().map(|pkg| pkg.name));
    run_command_inherit(&brew, &args)?;

    for pkg in missing {
        if resolve_tool(pkg.name, pkg.version_args).is_none() {
            return Err(trf(&YT_PATH, &[("tool", pkg.name)]));
        }
        println!("{}", trf(&YT_TOOL_INSTALLED, &[("tool", pkg.name)]));
    }

    Ok(())
}

fn extra_bin_dirs() -> Vec<String> {
    let mut dirs = vec![
        "/opt/homebrew/bin".to_string(),
        "/usr/local/bin".to_string(),
    ];
    if let Some(home) = std::env::var_os("HOME") {
        dirs.push(
            Path::new(&home)
                .join(".local/bin")
                .to_string_lossy()
                .into_owned(),
        );
    }
    dirs
}

fn augmented_path() -> String {
    let current = std::env::var("PATH").unwrap_or_default();
    format!("{}:{current}", extra_bin_dirs().join(":"))
}

fn resolve_tool(program: &str, version_args: &[&str]) -> Option<String> {
    if command_succeeds(program, version_args) {
        return Some(program.to_string());
    }

    for dir in extra_bin_dirs() {
        let path = format!("{dir}/{program}");
        if Path::new(&path).is_file() && command_succeeds(&path, version_args) {
            return Some(path);
        }
    }

    None
}

fn run_command_inherit(program: &str, args: &[&str]) -> Result<(), String> {
    let status = Command::new(program)
        .args(args)
        .env("PATH", augmented_path())
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
        .env("PATH", augmented_path())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}
