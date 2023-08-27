mod packets;

use std::net::UdpSocket;

use packets::{PacketHeader, PACKET_HEADER_SIZE};

fn main() -> std::io::Result<()> {
    let host = "192.168.1.6";
    let port = 20777;
    let address = format!("{host}:{port}");
    let mut buffer = [0; 60577];
    let socket = UdpSocket::bind(address)?;

    loop {
        let (size, _addr) = socket.recv_from(&mut buffer)?;
        let buffer = &buffer[..size];
        let packet_header = PacketHeader::from(&buffer[..PACKET_HEADER_SIZE]);
    }
}
