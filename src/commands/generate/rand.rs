use std::process::Command;

pub enum Encoding {
    Hex,
    Base64,
}

impl Encoding {
    fn flag(self) -> &'static str {
        match self {
            Self::Hex => "-hex",
            Self::Base64 => "-base64",
        }
    }
}

pub fn run(encoding: Encoding, bytes: u32) {
    if bytes == 0 {
        eprintln!("A byte-számnak nagyobbnak kell lennie nullánál.");
        std::process::exit(1);
    }

    let flag = encoding.flag();

    let output = Command::new("openssl")
        .args(["rand", flag, &bytes.to_string()])
        .output();

    match output {
        Ok(result) if result.status.success() => {
            let secret = String::from_utf8_lossy(&result.stdout);
            let secret = secret
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>();
            println!("{secret}");
        }
        Ok(result) => {
            let stderr = String::from_utf8_lossy(&result.stderr);
            eprintln!(
                "Az openssl rand {flag} {bytes} sikertelen volt (kód: {:?}): {stderr}",
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
