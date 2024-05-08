use std::ascii::AsciiExt;
use anyhow::{Result, anyhow};

pub fn parse_string(data: &[u8]) -> Result<String> {
    let mut iter = data.into_iter();
    let mut str = String::new();

    while let Some(&i) = iter.next() {
        for _ in 0..i {
            let c: char = iter
                .next()
                .ok_or(anyhow!("Wrong encoding."))?
                .to_owned()
                .into();
            str.push(c);
        }
        str.push(' ');
    }

    Ok(str)
}
