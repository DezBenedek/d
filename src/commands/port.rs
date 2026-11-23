use crate::i18n::{
    ERR_EXIT_CODE, ERR_START_PROGRAM, PORT_FREE, PORT_GONE, PORT_KILL_FAIL, PORT_KILLED,
    PORT_LISTENING, trf,
};
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Listener {
    pub pid: u32,
    pub name: String,
}

pub fn run_show(port: u16) {
    match listeners(port) {
        Ok(found) if found.is_empty() => {
            println!("{}", trf(&PORT_FREE, &[("port", &port.to_string())]))
        }
        Ok(found) => {
            println!("{}", trf(&PORT_LISTENING, &[("port", &port.to_string())]));
            for listener in &found {
                println!("  {} (PID {})", listener.name, listener.pid);
            }
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

pub fn run_kill(port: u16) {
    let found = match listeners(port) {
        Ok(found) => found,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    };
    if found.is_empty() {
        println!("{}", trf(&PORT_FREE, &[("port", &port.to_string())]));
        return;
    }

    for listener in &found {
        // `kill` fails when the process just exited on its own.
        if !signal(&listener.pid, "TERM") && !alive(listener.pid) {
            println!("{}", trf(&PORT_GONE, &[("pid", &listener.pid.to_string())]));
            continue;
        }
        wait_for_exit(listener.pid);
        if alive(listener.pid) {
            signal(&listener.pid, "KILL");
            wait_for_exit(listener.pid);
        }
        if alive(listener.pid) {
            eprintln!(
                "{}",
                trf(&PORT_KILL_FAIL, &[("pid", &listener.pid.to_string())])
            );
        } else {
            println!(
                "{}",
                trf(
                    &PORT_KILLED,
                    &[("name", &listener.name), ("pid", &listener.pid.to_string())]
                )
            );
        }
    }
}

/// Processes in LISTEN state on a TCP port (via `lsof`).
/// `lsof` exits with code 1 when nothing matches — that means "free", not an error.
pub(crate) fn listeners(port: u16) -> Result<Vec<Listener>, String> {
    let output = Command::new("lsof")
        .args([
            format!("-iTCP:{port}"),
            "-sTCP:LISTEN".to_string(),
            "-P".to_string(),
            "-n".to_string(),
        ])
        .output()
        .map_err(|error| {
            trf(
                &ERR_START_PROGRAM,
                &[("program", "lsof"), ("error", &error.to_string())],
            )
        })?;

    if !output.status.success() && output.status.code() != Some(1) {
        return Err(trf(
            &ERR_EXIT_CODE,
            &[("code", &format!("{:?}", output.status.code()))],
        ));
    }

    Ok(parse_lsof(&String::from_utf8_lossy(&output.stdout)))
}

fn parse_lsof(text: &str) -> Vec<Listener> {
    let mut found = Vec::new();
    for (index, line) in text.lines().enumerate() {
        if index == 0 {
            continue; // header row
        }
        let mut columns = line.split_whitespace();
        let (Some(name), Some(pid)) = (columns.next(), columns.next()) else {
            continue;
        };
        let Ok(pid) = pid.parse::<u32>() else {
            continue;
        };
        if !found.iter().any(|listener: &Listener| listener.pid == pid) {
            found.push(Listener {
                pid,
                name: name.to_string(),
            });
        }
    }
    found
}

fn signal(pid: &u32, signal: &str) -> bool {
    Command::new("kill")
        .arg(format!("-{signal}"))
        .arg(pid.to_string())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn alive(pid: u32) -> bool {
    Command::new("kill")
        .arg("-0")
        .arg(pid.to_string())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn wait_for_exit(pid: u32) {
    for _ in 0..8 {
        if !alive(pid) {
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "COMMAND   PID USER   FD   TYPE DEVICE SIZE/OFF NODE NAME\n\
         node    12345 dez  21u  IPv6 0xabc      0t0  TCP *:3000 (LISTEN)\n\
         node    12345 dez  22u  IPv6 0xdef      0t0  TCP *:3000 (LISTEN)\n\
         Python  23456 dez   3u  IPv4 0x123      0t0  TCP *:3000 (LISTEN)\n";

    #[test]
    fn lsof_parses_listeners_and_dedupes_pids() {
        let found = parse_lsof(SAMPLE);
        assert_eq!(
            found,
            vec![
                Listener {
                    pid: 12345,
                    name: "node".to_string()
                },
                Listener {
                    pid: 23456,
                    name: "Python".to_string()
                },
            ]
        );
    }

    #[test]
    fn lsof_handles_empty_and_header_only() {
        assert!(parse_lsof("").is_empty());
        assert!(
            parse_lsof("COMMAND   PID USER   FD   TYPE DEVICE SIZE/OFF NODE NAME\n").is_empty()
        );
    }

    #[test]
    fn lsof_skips_malformed_rows() {
        let found = parse_lsof("COMMAND   PID USER\nbroken-line\nnode notapid x\n");
        assert!(found.is_empty());
    }
}
