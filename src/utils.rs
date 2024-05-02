use std::ascii::AsciiExt;

pub fn parse_string(data: &[u8]) -> String {
    let mut iter = data.into_iter();
    let mut str = String::new();

    while let Some(&i) = iter.next() {
        for _ in 0..i {
            let c: char = iter.next().expect("Character expected.").to_owned().into();
            str.push(c);
        }
        str.push(' ');
    }

    str
}
