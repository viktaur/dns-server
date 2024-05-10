
use anyhow::{anyhow, Result};

#[derive(PartialEq, Clone, Copy, Debug, Hash, Eq, serde::Deserialize)]
pub enum RecordType {
    A,
    AAAA,
}

impl RecordType {
    pub fn encode(&self) -> u16 {
        match self {
            RecordType::A => 1,
            RecordType::AAAA => 28
        }
    }

    pub fn decode(id: u16) -> Result<Self> {
        match id {
            1 => Ok(RecordType::A),
            28 => Ok(RecordType::AAAA),
            _ => Err(anyhow!("Record type unsupported or unknown."))
        }
    }
}
