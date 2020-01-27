use std::io::prelude::*;
use std::io::{self, Result};
use std::net::{Ipv4Addr, Shutdown, SocketAddr, SocketAddrV4, TcpStream};
use std::process::Command;
use std::time::Duration;

fn main() -> Result<()> {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 4000));
    Command::new("clear").spawn()?.wait();

    if let Ok(mut stream) = TcpStream::connect(&addr) {
        println!("local addr: {}", stream.local_addr()?);
        println!("remote addr: {}", stream.peer_addr()?);

    // run_command_prompt(&mut stream);
    } else {
        println!("Couldn't connect to server...");
    }

    // tests require client to receive args
    // therefore adding temporary panic
    panic!("no args");
    Ok(())
}

fn run_command_prompt(stream: &mut TcpStream) -> Result<()> {
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush();

        io::stdin().read_line(&mut input)?;
        transmit_client_input(stream, input.trim());
    }
    Ok(())
}

fn transmit_client_input(stream: &mut TcpStream, input: &str) -> Result<usize> {
    stream.write(input.as_bytes())
}
