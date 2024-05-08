use crate::answer::Answer;
use crate::buffer::ByteBuffer;
use crate::header::{self, Header};
use crate::query::Query;
use anyhow::{Result};
use deku::prelude::*;

#[derive(Clone)]
pub struct DnsMessage {
    pub header: Header,
    pub queries: Vec<Query>,
    pub answers: Vec<Answer>,
}

impl DnsMessage {
    pub fn handle(&self, answers: Vec<Answer>) -> Result<Self> {
        let queries = self.queries.clone();
        let header = Header {
            transaction_id: self.header.transaction_id,
            flags: self.header.flags.handle()?,
            question: self.header.question,
            answer: self.answers.len() as u16,
            authority: self.header.authority,
            additional: self.header.additional,
        };

        // for query in queries {
            // query.name
        // }

        Ok(
            DnsMessage {
                header,
                queries,
                answers,
            }
        )
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self> {
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

impl Into<Vec<u8>> for DnsMessage {
    fn into(self) -> Vec<u8> {
        todo!()
    }
}
