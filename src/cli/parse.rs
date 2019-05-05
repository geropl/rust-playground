
use clap::{App, SubCommand, Arg};

pub fn parse_cmd<'a>() -> App<'a, 'a> {
    SubCommand::with_name("parse")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .takes_value(true)
            .help("The file to parse"))
}
