use deku::prelude::*;
use crate::buffer::ByteDecoder;
use anyhow::{anyhow, Error, Result};

#[derive(Debug, PartialEq, Clone, Copy)]
// #[deku(
//     ctx = "endian: deku::ctx::Endian",
//     endian = "endian"
// )]
pub struct Header {
    pub transaction_id: u16,
    pub flags: Flags,
    /// Number of queries in packet.
    pub question: u16,
    /// Number of answers in packet.
    pub answer: u16,
    /// Number of authoritative records in packet.
    pub authority: u16,
    /// Number of additional records in packet.
    pub additional: u16,
}

impl Header {
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut bytes = vec![];

        bytes.extend(self.transaction_id.to_be_bytes());
        bytes.extend(self.flags.to_bytes()?);
        bytes.extend(self.question.to_be_bytes());
        bytes.extend(self.answer.to_be_bytes());
        bytes.extend(self.authority.to_be_bytes());
        bytes.extend(self.additional.to_be_bytes());

        Ok(bytes)
    }

    pub fn decode(buf: &mut ByteDecoder) -> Result<Self> {
        let transaction_id = u16::from_be_bytes(buf.read_n_bytes(2)?.try_into()?);
        let ((_, _), flags) = Flags::from_bytes((buf.read_n_bytes(2)?, 0))?;
        let question = u16::from_be_bytes(buf.read_n_bytes(2)?.try_into()?);
        let answer = u16::from_be_bytes(buf.read_n_bytes(2)?.try_into()?);
        let authority = u16::from_be_bytes(buf.read_n_bytes(2)?.try_into()?);
        let additional = u16::from_be_bytes(buf.read_n_bytes(2)?.try_into()?);

        Ok(
            Header {
                transaction_id,
                flags,
                question,
                answer,
                authority,
                additional,
            }
        )
    }
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Clone, Copy)]
// #[deku(
//     // accept the endian variable from the parent
//     ctx = "endian: deku::ctx::Endian",
//     // use it as our struct's endian value
//     endian = "endian"
// )]
#[deku(endian="big")]
pub struct Flags {
    /// Indicates if the message is a query (0) or a reply (1).
    #[deku(bits=1)]
    pub qr: bool,

    /// The type can be QUERY (standard query, 0), IQUERY (inverse query, 1), or STATUS
    /// (server status request, 2).
    #[deku(bits=4)]
    pub opcode: u8,

    /// Authoritative Answer, in a response, indicates if the DNS server is authoritative
    /// for the queried hostname.
    #[deku(bits=1)]
    pub aa: bool,

    /// Truncation, indicates the message was truncated due to excessive length.
    #[deku(bits=1)]
    pub tc: bool,

    /// Recursion desired, indicates if the client means a recursive query.
    #[deku(bits=1)]
    pub rd: bool,

    /// Recursion available, in a response, indicates if the replying DNS server
    /// supports recursion.
    #[deku(bits=1)]
    pub ra: bool,

    /// Zero, reserved for future use.
    #[deku(bits=3)]
    pub z: u8,

    /// Reponse code, can be NOERROR (0), FORMERR (1, Format error), SERVFAIL (2),
    /// NXDOMAIN (3, Nonexistent domain), etc.
    #[deku(bits=4)]
    pub rcode: u8
}

impl Flags {
    pub fn handle(&self) -> Result<Self> {
        let mut new_flags = self.clone();

        if !self.qr { // it's a query
            new_flags.qr = true;
        } else { // it's a response
            return Err(anyhow!("Header flags indicate message is not a query."));
        }

        match self.opcode {
            0 => (),
            _ => return Err(anyhow!("RCODE should always be 0 in queries."))
        }

        Ok(new_flags)
    }
}
