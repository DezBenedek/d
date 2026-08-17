use super::bytes::openssl_rand_bytes;
use crate::i18n::{GEN_RAND_SHORT, tr};

pub fn run() {
    match openssl_rand_bytes(16) {
        Ok(bytes) if bytes.len() >= 16 => {
            let mut buf = [0u8; 16];
            buf.copy_from_slice(&bytes[..16]);
            println!("{}", uuid_v4_from_bytes(buf));
        }
        Ok(_) => {
            eprintln!("{}", tr(&GEN_RAND_SHORT));
            std::process::exit(1);
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

pub(crate) fn uuid_v4_from_bytes(mut bytes: [u8; 16]) -> String {
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0],
        bytes[1],
        bytes[2],
        bytes[3],
        bytes[4],
        bytes[5],
        bytes[6],
        bytes[7],
        bytes[8],
        bytes[9],
        bytes[10],
        bytes[11],
        bytes[12],
        bytes[13],
        bytes[14],
        bytes[15]
    )
}

#[cfg(test)]
mod tests {
    use super::uuid_v4_from_bytes;

    #[test]
    fn uuid_v4_sets_version_and_variant() {
        assert_eq!(
            uuid_v4_from_bytes([0u8; 16]),
            "00000000-0000-4000-8000-000000000000"
        );

        let uuid = uuid_v4_from_bytes([0xff; 16]);
        assert_eq!(uuid, "ffffffff-ffff-4fff-bfff-ffffffffffff");
        assert_eq!(uuid.chars().nth(14), Some('4'));
        assert!(matches!(uuid.chars().nth(19), Some('8' | '9' | 'a' | 'b')));
    }
}
