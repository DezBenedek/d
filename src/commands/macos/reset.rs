use super::util::{killall, run_defaults};
use crate::i18n::{
    DONE, MACOS_BATTERY, MACOS_HIDDEN, MACOS_PATHBAR, MACOS_SET_ERR, MACOS_SET_OK, MACOS_STATUSBAR,
    tr, trf,
};

pub fn run() {
    let settings: [(&[&str], &str); 4] = [
        (
            &[
                "-currentHost",
                "write",
                "com.apple.controlcenter",
                "BatteryShowPercentage",
                "-bool",
                "false",
            ],
            tr(&MACOS_BATTERY),
        ),
        (
            &["write", "com.apple.finder", "ShowPathbar", "-bool", "false"],
            tr(&MACOS_PATHBAR),
        ),
        (
            &[
                "write",
                "com.apple.finder",
                "ShowStatusBar",
                "-bool",
                "false",
            ],
            tr(&MACOS_STATUSBAR),
        ),
        (
            &[
                "write",
                "com.apple.finder",
                "AppleShowAllFiles",
                "-bool",
                "false",
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

    killall("ControlCenter");
    killall("Finder");

    println!("{}", tr(&DONE));
}
