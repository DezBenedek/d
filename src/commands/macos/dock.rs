use super::util::{killall, run_defaults};
use crate::i18n::{DONE, MACOS_DOCK, MACOS_SET_ERR, MACOS_SET_OK, tr, trf};

pub fn run() {
    let label = tr(&MACOS_DOCK);
    if let Err(error) = run_defaults(&["write", "com.apple.dock", "autohide", "-bool", "true"]) {
        eprintln!(
            "{}",
            trf(&MACOS_SET_ERR, &[("label", label), ("error", &error)])
        );
        std::process::exit(1);
    }

    println!("{}", trf(&MACOS_SET_OK, &[("label", label)]));
    killall("Dock");
    println!("{}", tr(&DONE));
}
