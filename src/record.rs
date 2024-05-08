use deku::prelude::*;
use anyhow::{Error, anyhow, Result};

pub enum RecordType {
    A,
    AAAA,
}

impl TryFrom<u16> for RecordType {
    type Error = anyhow::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(RecordType::A),
            28 => Ok(RecordType::AAAA),
            _ => Err(anyhow!("Record type unsupported or unknown."))
        }
    }
}

impl Into<u16> for RecordType {
    fn into(self) -> u16 {
        match self {
            RecordType::A => 1,
            RecordType::AAAA => 28,
        }
    }
}


// TODO: name, type, class, ttl, rdlength, rdata
pub struct ResourceRecord {


}
