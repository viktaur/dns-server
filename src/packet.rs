use deku::prelude::*;
use crate::record::Record;
use crate::question::Question;

pub struct Packet {
    pub header: Header,
    pub question: Vec<Question>,
    pub answer: Vec<Record>,
    pub authority: Vec<Record>,
    pub additional: Vec<Record>
}

pub struct Header {
    pub transaction_id: u16,        // 2 bytes
    pub flags: Flags,               // 2 bytes
    pub question: u16,              // 2 bytes
    pub answer: u16,                // 2 bytes
    pub authority: u16,             // 2 bytes
    pub additional: u16,            // 2 bytes
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
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

    /// Recursion available, in a response, indivicates if the replying DNS server
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
