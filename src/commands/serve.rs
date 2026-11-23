use crate::commands::download::common::expand_path;
use crate::commands::ip::local_ip;
use crate::i18n::{ERR_START_PROGRAM, SERVE_NO_PYTHON, SERVE_SERVING, YT_NOT_A_DIR, tr, trf};
use std::process::{Command, Stdio};

const DEFAULT_PORT: u16 = 8000;

pub fn run(dir: Option<String>, port: Option<u16>) {
    let port = port.unwrap_or(DEFAULT_PORT);
    let raw = match dir {
        Some(value) if !value.trim().is_empty() => value.trim().to_string(),
        _ => ".".to_string(),
    };
    let path = expand_path(&raw);
    if !path.is_dir() {
        eprintln!(
            "{}",
            trf(&YT_NOT_A_DIR, &[("path", &path.display().to_string())])
        );
        std::process::exit(1);
    }

    let has_python = Command::new("python3")
        .args(["--version"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false);
    if !has_python {
        eprintln!("{}", tr(&SERVE_NO_PYTHON));
        std::process::exit(1);
    }

    let lan = local_ip().unwrap_or_else(|_| "n/a".to_string());
    println!(
        "{}",
        trf(
            &SERVE_SERVING,
            &[
                ("dir", &path.display().to_string()),
                ("url", &format!("http://localhost:{port}")),
                ("lan", &lan),
            ]
        )
    );

    let status = Command::new("python3")
        .args([
            "-m",
            "http.server",
            &port.to_string(),
            "--directory",
            &path.to_string_lossy(),
        ])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|error| {
            trf(
                &ERR_START_PROGRAM,
                &[("program", "python3"), ("error", &error.to_string())],
            )
        });

    match status {
        Ok(status) => std::process::exit(status.code().unwrap_or(1)),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
