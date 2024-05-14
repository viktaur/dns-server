
use anyhow::{anyhow, Result};

#[derive(PartialEq, Clone, Copy, Debug, Hash, Eq, serde::Deserialize)]
pub enum RecordType {
    A,
    AAAA,
    CNAME,
}

impl RecordType {
    pub fn encode(&self) -> u16 {
        match self {
            RecordType::A => 1,
            RecordType::AAAA => 28,
            RecordType::CNAME => 5,
        }
    }

    pub fn decode(id: u16) -> Result<Self> {
        match id {
            1 => Ok(RecordType::A),
            28 => Ok(RecordType::AAAA),
            5 => Ok(RecordType::CNAME),
            _ => Err(anyhow!("Record type unsupported or unknown."))
        }
    }
}
