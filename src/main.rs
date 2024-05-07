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


fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let port: u16 = args[1].parse()?;
    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], port)),
        // IPv6 address here.
    ];
    let socket = UdpSocket::bind(&addrs[..])?;

    // let socket = UdpSocket::bind("127.0.0.1:1053")?;
    println!("Listening on port {port}");

    loop {


        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut buf = [0; 1024];
        let (amt, src) = socket.recv_from(&mut buf)?;
        println!("Received {amt} bytes from {src}");

        // Redeclare `buf` as slice of the actually received data.
        let buf = &buf[..amt];
        let res = query(buf)?;
        socket.send_to(&res, &src)?;
        println!("Response sent to {src}");
    }
}

fn query(request: &[u8]) -> Result<Vec<u8>> {
    let mut f = File::open("response_packet.txt")?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    Ok(buf)
}


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


// fn main() -> Result<()> {
//     let args: Vec<String> = std::env::args().collect();
//     let port: u16 = args[1].parse()
//         .expect("Port should be a value between 0 and 65535.");

//     let socket = UdpSocket::bind(("127.0.0.1", port))?;
//     println!("Socket created and bound to port {}", port);

//     let mut buf = [0; 1024]; // Increased buffer size for receiving data
//     loop {
//         let (amt, src) = socket.recv_from(&mut buf)?;
//         println!("Received {} bytes from {}", amt, src);

//         // Read the contents of the file into a buffer
//         let mut f = File::open("response_packet.txt")?;
//         let mut buffer = Vec::new();
//         f.read_to_end(&mut buffer)?;

//         // Send the contents of the buffer back to the source address
//         socket.send_to(&buffer, &src)?;
//         println!("Sent {} bytes back to {}", buffer.len(), src);
//     }
// }
