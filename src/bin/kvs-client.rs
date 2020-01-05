use std::io::prelude::*;
use std::io::Result;
use std::net::TcpStream;

fn main() -> Result<()> {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:4000") {
        stream.write(&[1])?;
    } else {
        println!("Couldn't connect to server...");
    }

    Ok(())
}
