use std::io::prelude::*;
use std::io::Result;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream};
use std::time::Duration;

fn main() -> Result<()> {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 4000));
    if let Ok(mut stream) = TcpStream::connect(&addr) {
        println!("local addr: {}", stream.local_addr()?);
        println!("remote addr: {}", stream.peer_addr()?);

        let msg = b"write initiated";
        stream.write(&msg[..]);
    } else {
        println!("Couldn't connect to server...");
    }

    Ok(())
}
