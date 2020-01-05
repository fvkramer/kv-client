use clap::{App, Arg};
use slog::{Drain, *};
use slog_async;
use slog_term;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

const DEFAULT_ADDR: u16 = 4000;

fn handle_client(mut stream: TcpStream) {
    // Read all the headers
    stream.read(&mut [0; 128]);
}

fn main() -> Result<()> {
    let log_path = "target/devel-unified.log";
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(log_path)
        .unwrap();

    let decorator = slog_term::PlainDecorator::new(file);
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let _log = slog::Logger::root(drain, o!("key1" => "value1", "key2" => "value2"));

    info!(_log, "Server version number: {}", env!("CARGO_PKG_VERSION"));
    error!(_log, "{}", env!("CARGO_PKG_VERSION"));

    let matches = App::new("Kvs Server")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("address")
                .long("addr")
                .value_name("IP-PORT")
                .help("The port on which the server will bind")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("engine")
                .long("engine")
                .value_name("ENGINE-NAME")
                .help("engine used as kv server"),
        )
        .get_matches();

    if let Some(addr) = matches.value_of("address") {
        info!(_log, "Kvs-Server socket address is: {}", addr);
        error!(_log, "Kvs-Server socket address is: {}", addr);
        let listener = TcpListener::bind(format!("127.0.0.1:{}", addr))?;

        for stream in listener.incoming() {
            handle_client(stream.unwrap());
        }
    } else {
        info!(_log, "no process provided. Defaulting to 4000");
        error!(_log, "running on port 4000");
        let listener = TcpListener::bind(format!("127.0.0.1:{}", DEFAULT_ADDR))?;

        for stream in listener.incoming() {
            handle_client(stream.unwrap());
        }
    }

    if let Some(engine) = matches.value_of("engine") {
        info!(_log, "server running with: {} engine", engine);
        println!("received an engine: {}", engine)
    } else {
        info!(_log, "server running with: kvs engine");
        error!(_log, "server running with: kvs engine");
    }

    Ok(())
}
