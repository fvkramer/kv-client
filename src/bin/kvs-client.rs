use std::io::prelude::*;
use std::io::{self, Result};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream};
use std::process::Command;
use std::time::Duration;

fn main() -> Result<()> {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 4000));
    Command::new("clear").spawn()?.wait();

    run_command_prompt();
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

fn run_command_prompt() -> Result<()> {
    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush();

        io::stdin().read_line(&mut input)?;
        println!("{}", input.trim());
    }

    Ok(())
}
