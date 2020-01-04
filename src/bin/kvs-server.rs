use clap::{App, Arg};
use slog::{Drain, *};
use slog_async;
use slog_term;

use std::fs::OpenOptions;

fn main() {
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

    let _log = slog::Logger::root(drain, o!());

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
        println!("received an address: {}", addr)
    }

    if let Some(engine) = matches.value_of("engine") {
        println!("received an engine: {}", engine)
    }
}
