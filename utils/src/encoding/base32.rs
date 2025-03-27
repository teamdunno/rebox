const BASE32_ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

pub fn encode(to_encode: String) -> String {
    let bytes = to_encode.as_bytes();
    let mut encoded = String::new();
    let mut bits = 0u32;
    let mut bit_count = 0;

    for &byte in bytes {
        bits = (bits << 8) | byte as u32;
        bit_count += 8;
        while bit_count >= 5 {
            let index = (bits >> (bit_count - 5)) & 0b11111;
            encoded.push(BASE32_ALPHABET[index as usize] as char);
            bit_count -= 5;
        }
    }

    if bit_count > 0 {
        let index = (bits << (5 - bit_count)) & 0b11111;
        encoded.push(BASE32_ALPHABET[index as usize] as char);
    }

    while encoded.len() % 8 != 0 {
        encoded.push('=');
    }
    encoded
}

pub fn decode(to_decode: String) -> String {
    let bytes = to_decode.as_bytes();
    let mut decoded = Vec::new();
    let mut bits = 0u32;
    let mut bit_count = 0;

    for &b in bytes.iter().filter(|&&b| b != b'=') {
        if let Some(pos) = BASE32_ALPHABET.iter().position(|&c| c == b) {
            bits = (bits << 5) | pos as u32;
            bit_count += 5;
            if bit_count >= 8 {
                decoded.push((bits >> (bit_count - 8)) as u8);
                bit_count -= 8;
            }
        }
    }

    String::from_utf8(decoded).unwrap()
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_encode() {
        let encoded = encode("Hello, world!".to_string());
        assert_eq!(encoded, "JBSWY3DPFQQHO33SNRSCC===");
    }

    #[test]
    fn test_decode() {
        let decoded = decode("JBSWY3DPFQQHO33SNRSCC===".into());
        assert_eq!(decoded, "Hello, world!");
    }

    #[test]
    fn test_using_rfc() {
        assert_eq!(encode("".into()), "");
        assert_eq!(encode("f".into()), "MY======");
        assert_eq!(encode("fo".into()), "MZXQ====");
        assert_eq!(encode("foo".into()), "MZXW6===");
        assert_eq!(encode("foob".into()), "MZXW6YQ=");
        assert_eq!(encode("fooba".into()), "MZXW6YTB");
        assert_eq!(encode("foobar".into()), "MZXW6YTBOI======");
    }
}
