use deku::prelude::*;

pub enum Record {
    UNKNOWN {

    },
    A {
        domain: String,

    },
    AAAA {
        domain: String,

    },
    MX {
        hello: u8
    },
    TXT {

    }
}

pub enum RecordType {
    A,
    AAAA,
}

impl From<u16> for RecordType {
    fn from(value: u16) -> Self {
        match value {
            1 => RecordType::A,
            28 => RecordType::AAAA,
            _ => panic!("Record type unsupported or unknown.")
        }
    }
}

// TODO: name, type, class, ttl, rdlength, rdata
pub struct ResourceRecord {


}
