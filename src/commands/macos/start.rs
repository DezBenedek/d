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
            "akkumulátor százalék a menüsorban",
        ),
        (
            &["write", "com.apple.finder", "ShowPathbar", "-bool", "true"],
            "elérési út sáv (hol vagyok)",
        ),
        (
            &[
                "write",
                "com.apple.finder",
                "ShowStatusBar",
                "-bool",
                "true",
            ],
            "állapotsor (fájlméretek)",
        ),
        (
            &[
                "write",
                "com.apple.finder",
                "AppleShowAllFiles",
                "-bool",
                "true",
            ],
            "rejtett fájlok",
        ),
    ];

    for (args, label) in settings {
        if let Err(error) = run_defaults(args) {
            eprintln!("Nem sikerült beállítani ({label}): {error}");
            std::process::exit(1);
        }
        println!("Beállítva: {label}");
    }

    restart_menu_bar_and_finder();

    println!();
    println!("Kész.");
    println!(
        "Teljes mappaméret Finderben: nyiss meg egy mappát, Cmd+J, pipáld be a 'Calculate all sizes'-t, majd 'Use as Defaults'."
    );
}

fn run_defaults(args: &[&str]) -> Result<(), String> {
    let status = Command::new("defaults")
        .args(args)
        .status()
        .map_err(|error| format!("nem sikerült elindítani a defaults-ot: {error}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("kilépési kód: {:?}", status.code()))
    }
}

fn restart_menu_bar_and_finder() {
    let _ = Command::new("killall").arg("ControlCenter").status();
    let _ = Command::new("killall").arg("Finder").status();
}
