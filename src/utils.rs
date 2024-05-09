use std::ascii::AsciiExt;
use anyhow::{Result, anyhow};

pub fn parse_name(data: &[u8]) -> Result<(String, usize)> {
    let mut iter = data.into_iter();
    let mut name: Vec<String> = Vec::new();
    let mut bytes_read = 0;

    while let Some(&n) = iter.next() {
        bytes_read += 1;
        if n > 0 {
            let mut str = String::new();
            for _ in 0..n {
                bytes_read += 1;
                let c: char = iter
                    .next()
                    .ok_or(anyhow!("Wrong encoding."))?
                    .to_owned()
                    .into();
                str.push(c);
            }
            name.push(str);
        } else {
            break;
        }
    }

    Ok((name.join("."), bytes_read))
}

pub fn encode_name_offset(name: &str, message: &[u8]) -> Result<[u8; 2]> {
    let mut i = 0u8;

    while let Some(rest) = message.get(i as usize..) {
        if let Ok((parsed_name, _)) = parse_name(rest) {
            if parsed_name == name {
                return Ok([0xc0, i]);
            }
        }

        i += 1;
    }

    Err(anyhow!("Name not found in message."))
}

pub fn encode_name(name: &str) -> Result<Vec<u8>> {
    let fragments = name.split('.');
    let mut bytes: Vec<u8> = vec![];

    for fragment in fragments {
        bytes.push(fragment.len() as u8);
        bytes.extend(fragment.as_bytes());
    }
    bytes.push(0);

    Ok(bytes)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_name() {
        let bytes: Vec<u8> = vec![
            0x03, 0x77, 0x77, 0x77,
            0x07, 0x65, 0x78, 0x61, 0x6D, 0x70, 0x6C, 0x65,
            0x03, 0x63, 0x6F, 0x6D,
            0x00
        ];

        assert_eq!(parse_name(&bytes).unwrap(), ("www.example.com".into(), 17));
    }

    #[test]
    fn test_encode_name() {
        let bytes: Vec<u8> = vec![
            0x03, 0x77, 0x77, 0x77,
            0x07, 0x65, 0x78, 0x61, 0x6D, 0x70, 0x6C, 0x65,
            0x03, 0x63, 0x6F, 0x6D,
            0x00
        ];

        assert_eq!(encode_name("www.example.com").unwrap(), bytes);
    }

    #[test]
    fn test_encode_name_offset() {
        // Represents a query to "google.com"
        let data: Vec<u8> = vec![
            0x43, 0xb0, 0x01, 0x20, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06,
            0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0x01,
            0x00, 0x01,
        ];
        let encoding = encode_name_offset("google.com", &data).unwrap();
        assert_eq!(encoding, [0xc0, 0x0c]);
    }
}
