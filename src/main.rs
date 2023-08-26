mod packets;

use std::net::UdpSocket;

use packets::PacketHeader;


fn main() -> std::io::Result<()> {
    let host = "192.168.56.1";
    let port = 20777;
    let address = format!("{host}:{port}");
    let mut buffer = [0; 60577];
    let socket = UdpSocket::bind(address)?;

    loop {
        let (_size, _addr) = socket.recv_from(&mut buffer)?;
        println!("{:?}", &buffer);
    }
}
