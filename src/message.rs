use crate::answer::Answer;
use crate::buffer::ByteReader;
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
    pub fn handle(self, answers: Vec<Answer>) -> Result<Self> {
        let new_header = Header {
            transaction_id: self.header.transaction_id,
            flags: self.header.flags.handle()?,
            question: self.header.question,
            answer: self.answers.len() as u16,
            authority: self.header.authority,
            additional: self.header.additional,
        };
        let queries = self.queries.clone();

        Ok(
            DnsMessage {
                header: new_header,
                queries,
                answers,
            }
        )
    }

    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut buf = ByteReader::new(data);

        let header = Header::parse(&mut buf)?;
        let mut queries = vec![];
        let answers = vec![];

        for _ in 0..header.question {
            queries.push(Query::parse(&mut buf)?);
        };

        Ok(
            DnsMessage {
                header,
                queries,
                answers,
            }
        )
    }

    pub fn to_bytes(self) -> Result<Vec<u8>> {
        let mut bytes = vec![];

        bytes.extend(self.header.to_bytes()?);

        for query in self.queries {
            bytes.extend(query.to_bytes()?);
        }

        for answer in self.answers {
            bytes.extend(answer.to_bytes()?);
        }

        Ok(bytes)
    }
}
