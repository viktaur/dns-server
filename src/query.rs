use deku::prelude::*;
use anyhow::Result;
use crate::{buffer::ByteBuffer, header::Header, record::RecordType, utils::*};

#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite)]
pub struct Query {
    pub name: Name,
    pub record_type: u16,
    pub class: u16,
}

#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite)]
pub struct Name {

}

impl Query {
pub fn parse(buf: &mut ByteBuffer, header: Header) -> Result<Vec<Query>> {
        let mut v: Vec<Query> = vec![];

        for i in 0..(header.question) {
            let ((_, new_pos), data) = DekuContainerRead::from_bytes((buf.data(), buf.pos()))?;
            v.push(data);
            buf.jump_to(new_pos)?;
        }

        Ok(v)
    }

    fn get_name(&self) -> String {
        // parse_string(&self.name)
        todo!()
    }

    fn get_record_type(&self) -> Result<RecordType> {
        self.record_type.try_into()
    }

    fn get_class(&self) -> () {
        todo!()
    }
}
