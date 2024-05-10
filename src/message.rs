use crate::answer::Answer;
use crate::buffer::ByteDecoder;
use crate::header::Header;
use crate::query::Query;
use anyhow::Result;

pub struct DnsMessage {
    pub header: Header,
    pub queries: Vec<Query>,
    pub answers: Vec<Answer>,
}

impl DnsMessage {
    pub fn encode(self) -> Result<Vec<u8>> {
        let mut bytes = vec![];

        bytes.extend(self.header.encode()?);

        for query in self.queries {
            bytes.extend(query.encode()?);
        }

        for answer in self.answers {
            bytes.extend(answer.encode()?);
        }

        Ok(bytes)
    }

    pub fn decode(data: &[u8]) -> Result<Self> {
        let mut buf = ByteDecoder::new(data);

        let header = Header::decode(&mut buf)?;
        let mut queries = vec![];
        let mut answers = vec![];

        for _ in 0..header.question {
            queries.push(Query::decode(&mut buf)?);
        };

        // In case it's ever needed
        for _ in 0..header.answer {
            answers.push(Answer::decode(&mut buf)?);
        }

        Ok(
            DnsMessage {
                header,
                queries,
                answers,
            }
        )
    }

    pub fn handle(self, answers: Vec<Answer>) -> Result<Self> {
        let new_header = Header {
            transaction_id: self.header.transaction_id,
            flags: self.header.flags.handle()?,
            question: self.header.question,
            answer: answers.len() as u16,
            authority: self.header.authority,
            additional: self.header.additional,
        };
        let queries = self.queries.clone();

        Ok(
            DnsMessage {
                header: new_header,         // Replace the header with the new one.
                queries,                    // Queries section is left untouched.
                answers,                    // Include the answers section.
            }
        )
    }
}
