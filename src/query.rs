use anyhow::{Result};
use crate::{buffer::ByteDecoder, record::RecordType, utils};

#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    pub name: String,
    pub record_type: RecordType,
    pub class: u16,
}

impl Query {
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut bytes = vec![];

        bytes.extend(utils::encode_name(&self.name)?);
        bytes.extend(self.record_type.encode().to_be_bytes());
        bytes.extend(self.class.to_be_bytes());

        Ok(bytes)
    }

    pub fn decode(buf: &mut ByteDecoder) -> Result<Self> {
        let (name, bytes_read) = utils::parse_name(buf.slice())?;
        buf.step(bytes_read)?;

        let record_type = RecordType::decode(
            u16::from_be_bytes(buf.read_n_bytes(2)?.try_into()?)
        )?;

        let class = u16::from_be_bytes(buf.read_n_bytes(2)?.try_into()?);

        Ok(
            Query {
                name,
                record_type,
                class
            }
        )
    }
}
