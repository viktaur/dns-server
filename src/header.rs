use deku::prelude::*;
use crate::buffer::ByteReader;
use anyhow::{anyhow, Error, Result};

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
pub struct Header {
    #[deku(endian = "big")]
    pub transaction_id: u16,
    pub flags: Flags,
    /// Number of queries in packet.
    #[deku(endian = "big")]
    pub question: u16,
    /// Number of answers in packet.
    #[deku(endian = "big")]
    pub answer: u16,
    /// Number of authoritative records in packet.
    #[deku(endian = "big")]
    pub authority: u16,
    /// Number of additional records in packet.
    #[deku(endian = "big")]
    pub additional: u16,
}

impl Header {
    pub fn parse(buf: &mut ByteReader) -> Result<Self> {
        let ((remaining, _), header) = DekuContainerRead::from_bytes((buf.data(), buf.pos()*8))?;
        let bytes_read = buf.remaining_bytes() - remaining.len();
        buf.step(bytes_read)?;
        Ok(header)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(DekuContainerWrite::to_bytes(self)?)
    }
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Clone, Copy)]
#[deku(endian = "big")]
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
            _ => return Err(anyhow!("Something went wrong in the query!"))
        }

        Ok(new_flags)
    }
}
