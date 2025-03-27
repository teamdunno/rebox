const BASE16_ALPHABET: &[u8] = b"0123456789ABCDEF";

pub fn encode(to_encode: String) -> String {
    let bytes = to_encode.as_bytes();
    let mut encoded = String::new();

    for &byte in bytes {
        encoded.push(BASE16_ALPHABET[(byte >> 4) as usize] as char);
        encoded.push(BASE16_ALPHABET[(byte & 0x0F) as usize] as char);
    }
    encoded
}

pub fn decode(to_decode: String) -> String {
    let bytes = to_decode.as_bytes();

    let mut decoded = Vec::new();
    for chunk in bytes.chunks(2) {
        let high = BASE16_ALPHABET.iter().position(|&c| c == chunk[0]).unwrap() as u8;
        let low = BASE16_ALPHABET.iter().position(|&c| c == chunk[1]).unwrap() as u8;
        decoded.push((high << 4) | low);
    }

    String::from_utf8(decoded).unwrap()
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_encode() {
        let encoded = encode("Hello, world!".to_string());
        assert_eq!(encoded, "48656C6C6F2C20776F726C6421");
    }

    #[test]
    fn test_decode() {
        let decoded = decode("48656C6C6F2C20776F726C6421".into());
        assert_eq!(decoded, "Hello, world!");
    }

    #[test]
    fn test_using_rfc() {
        assert_eq!(encode("".into()), "");
        assert_eq!(encode("f".into()), "66");
        assert_eq!(encode("fo".into()), "666F");
        assert_eq!(encode("foo".into()), "666F6F");
        assert_eq!(encode("foob".into()), "666F6F62");
        assert_eq!(encode("fooba".into()), "666F6F6261");
        assert_eq!(encode("foobar".into()), "666F6F626172");
    }
}
