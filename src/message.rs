use crate::answer::Answer;
use crate::header::Header;
use crate::query::Query;
use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct DnsMessage {
    pub header: Header,
    pub queries: Vec<Query>,
    pub answers: Vec<Answer>,
}
