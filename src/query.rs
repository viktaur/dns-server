use deku::prelude::*;
use crate::{record::RecordType, utils::*};

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct Query {
    pub name: Vec<u8>,
    pub record_type: u16,
    pub class: u16,
}

impl Query {
    fn get_name(&self) -> String {
        parse_string(&self.name)
    }

    fn get_record_type(&self) -> RecordType {
        self.record_type.into()
    }

    fn get_class(&self) -> () {
        todo!()
    }
}
