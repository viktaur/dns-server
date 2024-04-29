use crate::header::Header;
use crate::record::Record;
use crate::question::Question;

pub struct Packet {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<Record>,
    pub authorities: Vec<Record>,
    pub resources: Vec<Record>
}
