use crate::i18n::{GEN_BYTES_ZERO, GEN_OPENSSL_FAIL, GEN_OPENSSL_START, tr, trf};
use std::process::Command;

pub fn openssl_rand_bytes(bytes: u32) -> Result<Vec<u8>, String> {
    if bytes == 0 {
        return Err(tr(&GEN_BYTES_ZERO).to_string());
    }

    let output = Command::new("openssl")
        .args(["rand", &bytes.to_string()])
        .output()
        .map_err(|error| trf(&GEN_OPENSSL_START, &[("error", &error.to_string())]))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(trf(
            &GEN_OPENSSL_FAIL,
            &[
                ("flag", ""),
                ("bytes", &bytes.to_string()),
                ("code", &format!("{:?}", output.status.code())),
                ("stderr", stderr.trim()),
            ],
        ));
    }

    Ok(output.stdout)
}
