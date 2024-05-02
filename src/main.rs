use anyhow::Result;
use message::{DnsMessage};
use std::{fs::File, io::Read};
use std::net::{SocketAddr, UdpSocket, Ipv4Addr, Ipv6Addr};

mod record;
mod message;
mod header;
mod query;
mod answer;
mod utils;
mod buffer;

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let args: Vec<String> = std::env::args().collect();
//     let port: u16 = args[1].parse()
//         .expect("Port should be a value between 0 and 65535.");
//     let addrs = [
//         SocketAddr::from(([127, 0, 0, 1], port))
//     ];

//     let socket = UdpSocket::bind(&addrs[..]).await?;
//     let mut

//     // Receives a single datagram message on the socket. If `buf` is too small to hold
//     // the message, it will be cut off.
//     let mut buf = [0; 10];
//     let (amt, src) = socket.recv_from(&mut buf)?;
// }


fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let port: u16 = args[1].parse()
        .expect("Port should be a value between 0 and 65535.");
    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], port))
    ];
    let socket = UdpSocket::bind(&addrs[..])?;
    println!("Socket created");

    let mut buf = [0; 10];
    let (amt, src) = socket.recv_from(&mut buf)?;

    let mut f = File::open("response_packet.txt")?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    socket.send_to(&buffer, &src)?;

    Ok(())
}
