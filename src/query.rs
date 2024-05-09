use deku::prelude::*;
use anyhow::{Result, anyhow};
use crate::{buffer::ByteReader, header::Header, record::RecordType, utils};

// #[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite)]
#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    pub name: String,
    pub record_type: RecordType,
    pub class: u16,
}

impl Query {
    pub fn parse(buf: &mut ByteReader) -> Result<Query> {
        let name = Self::parse_name(buf)?;
        let record_type = Self::parse_record_type(buf)?;
        let class = Self::parse_class(buf)?;

        Ok(
            Query {
                name,
                record_type,
                class,
            }
        )
    }

    fn parse_name(buf: &mut ByteReader) -> Result<String> {
        let (name, bytes_read) = utils::parse_name(buf.slice())?;
        buf.step(bytes_read)?;
        Ok(name)
    }

    fn parse_record_type(buf: &mut ByteReader) -> Result<RecordType> {
        // let bytes = buf.read_n_bytes(2)?;
        // let value = u16::from_be_bytes(bytes.try_into()?);
        // value.try_into()
        let ((_, new_pos), record_type) = DekuContainerRead::from_bytes((buf.data(), buf.pos()))?;
        buf.jump_to(new_pos)?;
        Ok(record_type)
    }

    fn parse_class(buf: &mut ByteReader) -> Result<u16> {
        let bytes = buf.read_n_bytes(2)?;
        let value = u16::from_be_bytes(bytes.try_into()?);
        Ok(value)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = vec![];

        bytes.extend(utils::encode_name(&self.name)?);
        bytes.extend(self.record_type.to_bytes()?);
        bytes.extend(u16::to_be_bytes(self.class));

        Ok(bytes)
    }
}
