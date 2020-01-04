use clap::{App, Arg};

fn main() {
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
