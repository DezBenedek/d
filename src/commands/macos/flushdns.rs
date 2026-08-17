use crate::i18n::{ERR_EXIT_CODE, ERR_START_PROGRAM, MACOS_FLUSHDNS_OK, tr, trf};
use std::process::Command;

pub fn run() {
    let status = Command::new("sudo")
        .args([
            "sh",
            "-c",
            "dscacheutil -flushcache && killall -HUP mDNSResponder",
        ])
        .status()
        .unwrap_or_else(|error| {
            eprintln!(
                "{}",
                trf(
                    &ERR_START_PROGRAM,
                    &[("program", "sudo"), ("error", &error.to_string())]
                )
            );
            std::process::exit(1);
        });

    if !status.success() {
        eprintln!(
            "{}",
            trf(&ERR_EXIT_CODE, &[("code", &format!("{:?}", status.code()))])
        );
        std::process::exit(1);
    }

    println!("{}", tr(&MACOS_FLUSHDNS_OK));
}
