use crate::commands::ip::local_ip;
use crate::i18n::{
    SYS_CORES, SYS_CPU, SYS_DISK, SYS_FREE, SYS_IP, SYS_MEMORY, SYS_OS, SYS_UPTIME, tr,
};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn run() {
    let os = os_line();
    let cpu = cpu_line();
    let memory = memory_line();
    let disk = disk_line();
    let uptime = uptime_line();
    let ip = local_ip().unwrap_or_else(|_| "n/a".to_string());

    println!("{}: {os}", tr(&SYS_OS));
    println!("{}: {cpu}", tr(&SYS_CPU));
    println!("{}: {memory}", tr(&SYS_MEMORY));
    println!("{}: {disk}", tr(&SYS_DISK));
    println!("{}: {uptime}", tr(&SYS_UPTIME));
    println!("{}: {ip}", tr(&SYS_IP));
}

fn sh(program: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(program).args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn os_line() -> String {
    let name = sh("sw_vers", &["-productName"]).unwrap_or_else(|| "macOS".to_string());
    let version = sh("sw_vers", &["-productVersion"]).unwrap_or_else(|| "n/a".to_string());
    let arch = sh("uname", &["-m"]).unwrap_or_else(|| "n/a".to_string());
    format!("{name} {version} ({arch})")
}

fn cpu_line() -> String {
    let brand =
        sh("sysctl", &["-n", "machdep.cpu.brand_string"]).unwrap_or_else(|| "n/a".to_string());
    let cores = sh("sysctl", &["-n", "hw.ncpu"]).unwrap_or_else(|| "?".to_string());
    format!("{brand} ({} {})", cores.trim(), tr(&SYS_CORES))
}

fn memory_line() -> String {
    match sh("sysctl", &["-n", "hw.memsize"]).and_then(|value| value.parse::<u64>().ok()) {
        Some(bytes) => format_gb(bytes),
        None => "n/a".to_string(),
    }
}

fn disk_line() -> String {
    match sh("df", &["-k", "/"]).and_then(|out| parse_df_available(&out)) {
        Some(bytes) => format!("{} {}", format_gb(bytes), tr(&SYS_FREE)),
        None => "n/a".to_string(),
    }
}

fn uptime_line() -> String {
    let boot = sh("sysctl", &["-n", "kern.boottime"]).and_then(|out| parse_boottime(&out));
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    match boot {
        Some(boot) if now >= boot => format_duration(now - boot),
        _ => "n/a".to_string(),
    }
}

fn format_gb(bytes: u64) -> String {
    format!("{:.1} GB", bytes as f64 / 1024.0 / 1024.0 / 1024.0)
}

/// Available bytes on `/` from `df -k` output (4th column of the data row).
fn parse_df_available(output: &str) -> Option<u64> {
    let row = output.lines().nth(1)?;
    let available = row.split_whitespace().nth(3)?;
    available.parse::<u64>().ok().map(|kib| kib * 1024)
}

/// Boot epoch from `sysctl kern.boottime`, e.g. `{ sec = 1762000000, usec = 0 } ...`.
fn parse_boottime(output: &str) -> Option<u64> {
    let rest = output.split("sec = ").nth(1)?;
    rest.split(|c: char| !c.is_ascii_digit())
        .next()?
        .parse::<u64>()
        .ok()
}

fn format_duration(total_secs: u64) -> String {
    let days = total_secs / 86_400;
    let hours = (total_secs % 86_400) / 3_600;
    let minutes = (total_secs % 3_600) / 60;
    if days > 0 {
        format!("{days}d {hours}h {minutes}m")
    } else if hours > 0 {
        format!("{hours}h {minutes}m")
    } else if minutes > 0 {
        format!("{minutes}m")
    } else {
        format!("{total_secs}s")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gb_formats_with_one_decimal() {
        assert_eq!(format_gb(16 * 1024 * 1024 * 1024), "16.0 GB");
        assert_eq!(format_gb(123_456_789), "0.1 GB");
    }

    #[test]
    fn df_parses_available_column() {
        let output = "Filesystem   1024-blocks      Used Available Capacity Mounted on\n\
             /dev/disk3s1  488245288 300000000 150000000    67%    /\n";
        assert_eq!(parse_df_available(output), Some(150_000_000 * 1024));
        assert_eq!(parse_df_available("Filesystem   1024-blocks\n"), None);
        assert_eq!(parse_df_available(""), None);
    }

    #[test]
    fn boottime_parses_sec_field() {
        assert_eq!(
            parse_boottime("{ sec = 1762000000, usec = 0 } Thu Nov 20 2026"),
            Some(1_762_000_000)
        );
        assert_eq!(parse_boottime("garbage"), None);
    }

    #[test]
    fn duration_picks_largest_units() {
        assert_eq!(
            format_duration(3 * 86_400 + 4 * 3_600 + 12 * 60),
            "3d 4h 12m"
        );
        assert_eq!(format_duration(4 * 3_600 + 5 * 60), "4h 5m");
        assert_eq!(format_duration(90), "1m");
        assert_eq!(format_duration(45), "45s");
    }
}
