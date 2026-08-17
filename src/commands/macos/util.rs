use crate::i18n::{ERR_EXIT_CODE, ERR_START_PROGRAM, trf};
use std::process::Command;

pub fn run_defaults(args: &[&str]) -> Result<(), String> {
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

pub fn killall(name: &str) {
    let _ = Command::new("killall").arg(name).status();
}
