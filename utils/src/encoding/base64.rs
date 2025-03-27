const BASE64_ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode(to_encode: String) -> String {
    let bytes = to_encode.as_bytes();
    let mut encoded = String::new();
    let mut chunks = bytes.chunks_exact(3);

    for chunk in &mut chunks {
        let b1 = chunk[0] as u32;
        let b2 = chunk[1] as u32;
        let b3 = chunk[2] as u32;

        encoded.push(BASE64_ALPHABET[(b1 >> 2) as usize] as char);
        encoded.push(BASE64_ALPHABET[((b1 & 0b11) << 4 | (b2 >> 4)) as usize] as char);
        encoded.push(BASE64_ALPHABET[((b2 & 0b1111) << 2 | (b3 >> 6)) as usize] as char);
        encoded.push(BASE64_ALPHABET[(b3 & 0b111111) as usize] as char);
    }

    let remainder = chunks.remainder();
    if !remainder.is_empty() {
        let b1 = remainder[0] as u32;
        encoded.push(BASE64_ALPHABET[(b1 >> 2) as usize] as char);
        if remainder.len() == 2 {
            let b2 = remainder[1] as u32;
            encoded.push(BASE64_ALPHABET[((b1 & 0b11) << 4 | (b2 >> 4)) as usize] as char);
            encoded.push(BASE64_ALPHABET[((b2 & 0b1111) << 2) as usize] as char);
            encoded.push('=');
        } else {
            encoded.push(BASE64_ALPHABET[((b1 & 0b11) << 4) as usize] as char);
            encoded.push_str("==");
        }
    }
    encoded
}

pub fn decode(to_decode: String) -> String {
    let bytes = to_decode.as_bytes();

    let mut decoded = Vec::new();
    let mut buffer = [0u8; 4];
    let mut index = 0;

    for &b in bytes.iter().filter(|&&b| b != b'=') {
        if let Some(pos) = BASE64_ALPHABET.iter().position(|&c| c == b) {
            buffer[index] = pos as u8;
            index += 1;
            if index == 4 {
                decoded.push(buffer[0] << 2 | buffer[1] >> 4);
                decoded.push(buffer[1] << 4 | buffer[2] >> 2);
                decoded.push(buffer[2] << 6 | buffer[3]);
                index = 0;
            }
        }
    }

    if index > 1 {
        decoded.push(buffer[0] << 2 | buffer[1] >> 4);
    }
    if index > 2 {
        decoded.push(buffer[1] << 4 | buffer[2] >> 2);
    }

    String::from_utf8(decoded).unwrap()
}

mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let encoded = encode("Hello, world!".to_string());
        assert_eq!(encoded, "SGVsbG8sIHdvcmxkIQ==");
    }

    #[test]
    fn test_decode() {
        let decoded = decode("SGVsbG8sIHdvcmxkIQ==".into());
        assert_eq!(decoded, "Hello, world!");
    }

    #[test]
    fn test_using_rfc() {
        assert_eq!(encode("".into()), "");
        assert_eq!(encode("f".into()), "Zg==");
        assert_eq!(encode("fo".into()), "Zm8=");
        assert_eq!(encode("foo".into()), "Zm9v");
        assert_eq!(encode("foob".into()), "Zm9vYg==");
        assert_eq!(encode("fooba".into()), "Zm9vYmE=");
        assert_eq!(encode("foobar".into()), "Zm9vYmFy");
    }
}
