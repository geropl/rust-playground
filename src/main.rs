#[macro_use]
extern crate nom;
//#[macro_use]
extern crate clap;

mod ast;
mod parser;
mod cli;
use cli::{Cli};

fn main() {
    let cli = Cli::new();
    cli.run();
}
