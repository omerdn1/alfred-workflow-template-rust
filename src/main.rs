use clap::{App, Arg};
use std::io;

fn main() {
    let args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("query").required(true).index(1))
        .get_matches();

    let query = args.value_of("query").unwrap();

    alfred::json::Builder::with_items(&alfred_items)
            .write(io::stdout())
            .expect("Couldn't write items to Alfred");
}
