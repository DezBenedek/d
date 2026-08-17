use super::bytes::openssl_rand_bytes;
use crate::i18n::{GEN_BYTES_ZERO, tr};

const CHARSET: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*-_=+";

pub fn run(length: u32) {
    if length == 0 {
        eprintln!("{}", tr(&GEN_BYTES_ZERO));
        std::process::exit(1);
    }

    match openssl_rand_bytes(length) {
        Ok(bytes) => println!("{}", password_from_bytes(&bytes, CHARSET)),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

pub(crate) fn password_from_bytes(bytes: &[u8], charset: &[u8]) -> String {
    bytes
        .iter()
        .map(|&byte| charset[byte as usize % charset.len()] as char)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::password_from_bytes;

    #[test]
    fn password_from_bytes_maps_into_charset() {
        assert_eq!(password_from_bytes(&[0, 1, 2], b"ABC"), "ABC");
        assert_eq!(password_from_bytes(&[3, 4], b"ABC"), "AB");
        assert_eq!(password_from_bytes(&[], b"ABC"), "");
    }

    #[test]
    fn password_length_matches_input() {
        let bytes: Vec<u8> = (0..24).collect();
        assert_eq!(password_from_bytes(&bytes, super::CHARSET).len(), 24);
    }
}
