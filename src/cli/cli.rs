use std::process::exit;
use clap::{App};

use parser;
use cli::parse::parse_cmd;

use nom::types::CompleteStr;

pub struct Cli<'a> {
    pub app: App<'a, 'a>
}

impl<'a> Cli<'a> {
    pub fn new() -> Cli<'a> {
        Cli {
            app: App::new("cli")
            .version("0.1.0")
            .author("Gero Posmyk-Leinemann")
            .about("Some cli test")
            .subcommand(parse_cmd())
        }
    }

    pub fn run(self) -> () {
        let matches = self.app.get_matches();

        if let Some(ref parse_matches) = matches.subcommand_matches("parse") {
            // let file_value = parse_matches.value_of("file");
            // if None == file_value {
            //     println!("Invalid/no file path");
            //     exit(1);
            // }

            match parse_matches.value_of("file") {
                None => {
                    println!("Invalid/no file path");
                    exit(1);
                }
                Some(filepath) => {
                    let content = read_file_content(&filepath);
                    let result = parser::statement_list(CompleteStr::from(content.as_str()));
                    parser::dump(&result);

                    exit(0);
                }
            }
        }

        exit(1);
    }
}

fn read_file_content(filepath: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open(filepath).expect("File not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    contents
}
