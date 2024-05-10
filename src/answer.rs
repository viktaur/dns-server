use crate::{buffer::ByteDecoder, header::Header, query::Query, record::RecordType, utils};
use std::net::{Ipv4Addr, Ipv6Addr};
use anyhow::{Result};
use deku::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Answer {
    // For convenience and to avoid dependence on a context, this field is a u16 instead
    // of a String.
    name: u16,
    record_type: RecordType,
    class: u16,
    ttl: u32,
    data_length: u16,
    data: Vec<u8>
}

impl Answer {
    pub fn from_query(query: &Query, msg_bytes: &[u8], resource: &str) -> Result<Self> {
        let name_enc = utils::encode_name_offset(&query.name, msg_bytes)?;
        let name = u16::from_be_bytes(name_enc);
        let ttl = 0;
        let data: Vec<u8> = match query.record_type {
            RecordType::A => resource.parse::<Ipv4Addr>()?.octets().into(),
            RecordType::AAAA => resource.parse::<Ipv6Addr>()?.octets().into(),
        };

        Ok(
            Answer {
                name,
                record_type: query.record_type,
                class: query.class,
                ttl,
                data_length: data.len() as u16,
                data,
            }
        )
    }

    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut bytes = vec![];

        bytes.extend(self.name.to_be_bytes());
        bytes.extend(self.record_type.encode().to_be_bytes());
        bytes.extend(self.class.to_be_bytes());
        bytes.extend(self.ttl.to_be_bytes());
        bytes.extend(self.data_length.to_be_bytes());
        bytes.extend(&self.data);

        Ok(bytes)
    }

    pub fn decode(_buf: &mut ByteDecoder) -> Result<Self> {
        unreachable!()
    }
}
