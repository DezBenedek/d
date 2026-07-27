use std::process::Command;

pub fn run(bytes: u32) {
    if bytes == 0 {
        eprintln!("A byte-számnak nagyobbnak kell lennie nullánál.");
        std::process::exit(1);
    }

    let output = Command::new("openssl")
        .args(["rand", "-hex", &bytes.to_string()])
        .output();

    match output {
        Ok(result) if result.status.success() => {
            let secret = String::from_utf8_lossy(&result.stdout);
            print!("{secret}");
            if !secret.ends_with('\n') {
                println!();
            }
        }
        Ok(result) => {
            let stderr = String::from_utf8_lossy(&result.stderr);
            eprintln!(
                "Az openssl rand -hex {bytes} sikertelen volt (kód: {:?}): {stderr}",
                result.status.code()
            );
            std::process::exit(1);
        }
        Err(error) => {
            eprintln!("Nem sikerült elindítani az openssl-t: {error}");
            std::process::exit(1);
        }
    }
}
