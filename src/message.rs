use crate::answer::Answer;
use crate::buffer::ByteBuffer;
use crate::header::{self, Header};
use crate::query::Query;
use anyhow::{Result};
use deku::prelude::*;

pub struct DnsMessage {
    pub header: Header,
    pub queries: Vec<Query>,
    pub answers: Vec<Answer>,
}

impl DnsMessage {
    fn parse_from_bytes(data: &[u8]) -> Result<Self> {
        let mut buf = ByteBuffer::new();

        let header = Header::parse(&mut buf)?;
        let queries = Query::parse(&mut buf, header)?;
        let answers = Answer::parse(&mut buf, header)?;

        Ok(
            DnsMessage {
                header,
                queries,
                answers,
            }
        )
    }
}
