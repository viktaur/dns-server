use crate::{buffer::ByteBuffer, header::Header, record::{RecordType}, utils::parse_string};
use std::net::{Ipv4Addr, Ipv6Addr};
use anyhow::Result;
use deku::prelude::*;

#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite)]
pub struct Answer {
    name: u16,              // Represented as the offset, such as 0xc00c
    record_type: u16,       // e.g. AAAA 0x001x
    class: u16,
    ttl: u32,
    data_length: u16,
    #[deku(count = "data_length")]
    data: Vec<u8>           // e.g. 2a00:1440:4007::810d:2013
}

impl Answer {
    pub fn parse(buf: &mut ByteBuffer, header: Header) -> Result<Vec<Answer>> {
        let mut v: Vec<Answer> = vec![];

        for i in 0..(header.answer) {
            let ((_, new_pos), data) = DekuContainerRead::from_bytes((buf.data(), buf.pos()))?;
            v.push(data);
            buf.jump_to(new_pos)?;
        }

        Ok(v)
    }

    fn get_name(&self, message: &[u8]) -> Result<String> {
        let mut iter = message.into_iter();
        let offset = self.name - 0xc000;

        (0..offset).for_each(|_| {iter.next();});

        Ok(parse_string(iter.as_slice())?)
    }

    fn get_record_type(&self) -> Result<RecordType> {
        self.record_type.try_into()
    }

    fn get_class(&self) {
        todo!()
    }

    /// Time to live, number of seconds this record can live.
    fn get_ttl(&self) -> u32 {
        self.ttl
    }

    fn get_data(&self) -> Result<String> {
        match self.get_record_type()? {
            RecordType::A => {
                let array_bytes: [u8; 4] = self.data
                    .clone()
                    .try_into()
                    .expect("Data should contain exactly 4 bytes.");
                Ok(Ipv4Addr::from(array_bytes).to_string())
            },
            RecordType::AAAA => {
                let array_bytes: [u8; 16] = self.data
                    .clone()
                    .try_into()
                    .expect("Data should contain 16 bytes.");
                Ok(Ipv6Addr::from(array_bytes).to_string())
            }
        }
    }

}
