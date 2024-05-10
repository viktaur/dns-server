use answer::Answer;
use anyhow::{anyhow, Result};
use header::Header;
use message::{DnsMessage};
use record::RecordType;
use std::collections::HashMap;
use std::hash::Hash;
use std::io::BufReader;
use std::{fs::File, io::Read};
use std::net::{SocketAddr, UdpSocket};

mod record;
mod message;
mod header;
mod query;
mod answer;
mod utils;
mod buffer;

type NameToRecords = HashMap<String, HashMap<RecordType, Vec<String>>>;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let port: u16 = args[1].parse()?;
    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], port)),
        SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 1], port))
    ];
    let socket = UdpSocket::bind(&addrs[..])?;

    println!("Listening on port {port}");

    loop {
        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut buf = [0; 1024];
        let (amt, src) = socket.recv_from(&mut buf)?;
        println!("Received {amt} bytes from {src}");

        // Redeclare `buf` as slice of the actually received data.
        let buf = &buf[..amt];
        let res = match query(buf) {
            Ok(res) => res,
            Err(e) => { println!("{:?}", e); continue }
        };
        socket.send_to(&res, &src)?;
        println!("Response sent to {src}");
        // println!("Request: {:?}", buf);
        // println!("Response: {:?}", res);
    }
}

/// Processes a query represented as a binary DNS message and returns a response in the
/// same form.
fn query(request: &[u8]) -> Result<Vec<u8>> {
    let reader = BufReader::new(File::open("records.json")?);
    let records: NameToRecords = serde_json::from_reader(reader)?;
    let dns_msg = DnsMessage::decode(request)?;
    let mut answers = vec![];

    for query in &dns_msg.queries {
        let rrs = records
            .get(&query.name).ok_or(anyhow!("Name not found"))?
            .get(&query.record_type).ok_or(anyhow!("Query not found"))?;

        for rr in rrs {
            let answer = Answer::from_query(&query, request, rr)?;
            answers.push(answer);
        }
    }

    println!("{:?}", dns_msg.queries);
    println!("{:?}", answers);

    Ok(dns_msg.handle(answers)?.encode()?)
}
