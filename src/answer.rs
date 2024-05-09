use crate::{buffer::ByteReader, header::Header, query::Query, record::RecordType, utils};
use std::net::{Ipv4Addr, Ipv6Addr};
use anyhow::{Result, anyhow};
use deku::prelude::*;

#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite)]
pub struct Answer {
    name: u16,              // Represented as the offset, such as 0xc00c
    record_type: RecordType,       // e.g. AAAA 0x001x
    class: u16,
    ttl: u32,
    data_length: u16,
    #[deku(count = "data_length")]
    data: Vec<u8>           // e.g. 2a00:1440:4007::810d:2013
}

impl Answer {
    // Not really needed
    // pub fn parse(buf: &mut ByteBuffer, header: Header) -> Result<Answer> {
    //     let ((_, new_pos), data) = DekuContainerRead::from_bytes((buf.data(), buf.pos()))?;
    //     buf.jump_to(new_pos)?;

    //     Ok(data)
    // }

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

    pub fn to_bytes(self) -> Result<Vec<u8>> {
        Ok(DekuContainerWrite::to_bytes(&self)?)
    }

    // fn get_name(&self, message: &[u8]) -> Result<String> {
    //     let mut iter = message.into_iter();
    //     let offset = self.name - 0xc000;

    //     (0..offset).for_each(|_| {iter.next();});

    //     Ok(parse_name(iter.as_slice())?)
    // }

    // pub fn encode_name(&mut self, name: &str, message: &[u8]) -> Result<u16> {
    //     let mut i = 0u16;

    //     while let Some(rest) = message.get(i as usize..) {
    //         if let Ok(parsed_name) = utils::parse_name(rest) {
    //             if parsed_name == name {
    //                 return Ok(0xc000 + i);
    //             }
    //         }

    //         i += 1;
    //     }

    //     Err(anyhow!("Name not found in message."))
    // }

    // fn get_record_type(&self) -> Result<RecordType> {
    //     self.record_type.try_into()
    // }

    // fn get_class(&self) {
    //     todo!()
    // }

    /// Time to live, number of seconds this record can live.
    fn get_ttl(&self) -> u32 {
        self.ttl
    }

    // fn get_data(&self) -> Result<String> {
    //     match self.get_record_type()? {
    //         RecordType::A => {
    //             let array_bytes: [u8; 4] = self.data
    //                 .clone()
    //                 .try_into()
    //                 .expect("Data should contain exactly 4 bytes.");
    //             Ok(Ipv4Addr::from(array_bytes).to_string())
    //         },
    //         RecordType::AAAA => {
    //             let array_bytes: [u8; 16] = self.data
    //                 .clone()
    //                 .try_into()
    //                 .expect("Data should contain 16 bytes.");
    //             Ok(Ipv6Addr::from(array_bytes).to_string())
    //         }
    //     }
    // }
}
