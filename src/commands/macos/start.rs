use crate::i18n::{
    tr, trf, DONE, ERR_EXIT_CODE, ERR_START_PROGRAM, MACOS_BATTERY, MACOS_HIDDEN, MACOS_HINT,
    MACOS_PATHBAR, MACOS_SET_ERR, MACOS_SET_OK, MACOS_STATUSBAR,
};
use std::process::Command;

pub fn run() {
    let settings: [(&[&str], &str); 4] = [
        (
            &[
                "-currentHost",
                "write",
                "com.apple.controlcenter",
                "BatteryShowPercentage",
                "-bool",
                "true",
            ],
            tr(&MACOS_BATTERY),
        ),
        (
            &["write", "com.apple.finder", "ShowPathbar", "-bool", "true"],
            tr(&MACOS_PATHBAR),
        ),
        (
            &[
                "write",
                "com.apple.finder",
                "ShowStatusBar",
                "-bool",
                "true",
            ],
            tr(&MACOS_STATUSBAR),
        ),
        (
            &[
                "write",
                "com.apple.finder",
                "AppleShowAllFiles",
                "-bool",
                "true",
            ],
            tr(&MACOS_HIDDEN),
        ),
    ];

    for (args, label) in settings {
        if let Err(error) = run_defaults(args) {
            eprintln!(
                "{}",
                trf(&MACOS_SET_ERR, &[("label", label), ("error", &error)])
            );
            std::process::exit(1);
        }
        println!("{}", trf(&MACOS_SET_OK, &[("label", label)]));
    }

    restart_menu_bar_and_finder();

    println!();
    println!("{}", tr(&DONE));
    println!("{}", tr(&MACOS_HINT));
}

fn run_defaults(args: &[&str]) -> Result<(), String> {
    let status = Command::new("defaults")
        .args(args)
        .status()
        .map_err(|error| {
            trf(
                &ERR_START_PROGRAM,
                &[("program", "defaults"), ("error", &error.to_string())],
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

fn restart_menu_bar_and_finder() {
    let _ = Command::new("killall").arg("ControlCenter").status();
    let _ = Command::new("killall").arg("Finder").status();
}
