#[macro_use]
extern crate clap;
use clap::{Arg, App};

#[macro_use]
extern crate nom;
use nom::{IResult};

use std::fmt::Debug;


/*
 * <pattern>: <pattern_name> '=' <pattern>.
 * <pattern>: <structured_pattern> | <simple_pattern>
 * <structured_pattern>: '{' <simple_pattern> '}' [':' EOL]
 * <simple_pattern>: <string>
 */
#[derive(Debug, PartialEq)]
pub enum Equation<'a> {
    NamedEquation { name: &'a str, pattern: Pattern<'a> },
}

#[derive(Debug, PartialEq)]
pub enum Pattern<'a> {
    StructuredPattern { symbols: Vec<Symbol<'a>> },
    NamedPattern { name: &'a str },
}

#[derive(Debug, PartialEq)]
pub enum Quantifier {
    One,
    OneOrMore,
    ZeroOrMore,
}

#[derive(Debug, PartialEq)]
pub enum Symbol<'a> {
    Terminal { quantifier: Quantifier, value: &'a str },
    NonTerminal { quantifier: Quantifier },
}

named!(pattern_name<&str, &str>,
    ws!(
        nom::alpha
    )
);

named!(quantifier<&str, Quantifier>,
    alt!(
        do_parse!(
            tag_s!("+") >> (Quantifier::OneOrMore)
        ) |
        do_parse!(
            tag_s!("*") >> (Quantifier::ZeroOrMore)
        ) |
        do_parse!(
            (Quantifier::One)
        )
    )
);

named!(symbol<&str, Symbol>,
    do_parse!(
        value: pattern_name >>
        quantifier: quantifier >>
        (Symbol::Terminal { quantifier, value })
    )
);

named!(symbols<&str, Vec<Symbol>>,
    many1!(symbol)
);

named!(pattern<&str, Pattern>,
    alt!(
        do_parse!(
                ws!( tag_s!("{") ) >>
            symbols: symbols >>
                ws!( tag_s!("}") ) >>
            (Pattern::StructuredPattern { symbols })
        ) |
        do_parse!(
            name: pattern_name >>
            (Pattern::NamedPattern { name })
        )
    )
);

named!(equation<&str, Equation>,
    do_parse!(
        name: pattern_name >>
            tag_s!("=") >>
        pattern: pattern >>
            tag_s!(".") >>
        (Equation::NamedEquation { name, pattern })
    )
);

#[test]
fn parse_structured_equation() {
    let result = equation("URI = { asd }.");
    dump(&result);
    assert_eq!(result, Ok(("", Equation::NamedEquation {
        name: "URI",
        pattern: Pattern::StructuredPattern {
            symbols: vec![Symbol::Terminal {
                quantifier: Quantifier::One,
                value: "asd"
            }]
        }
    })));
}

#[test]
fn parse_structured_equation_one_or_more() {
    let result = equation("URI = { asd+ }.");
    dump(&result);
    assert_eq!(result, Ok(("", Equation::NamedEquation {
        name: "URI",
        pattern: Pattern::StructuredPattern {
            symbols: vec![Symbol::Terminal {
                quantifier: Quantifier::OneOrMore,
                value: "asd"
            }]
        }
    })));
}
#[test]
fn parse_structured_equation_zero_or_more() {
    let result = equation("URI = { asd* }.");
    dump(&result);
    assert_eq!(result, Ok(("", Equation::NamedEquation {
        name: "URI",
        pattern: Pattern::StructuredPattern {
            symbols: vec![Symbol::Terminal {
                quantifier: Quantifier::ZeroOrMore,
                value: "asd"
            }]
        }
    })));
}

#[test]
fn parse_named_equation() {
    let result = equation("URI = URL.");
    dump(&result);
    assert_eq!(result, Ok(("", Equation::NamedEquation {
        name: "URI",
        pattern: Pattern::NamedPattern {
            name: "URL"
        }
    })));
}

fn dump<T: Debug>(res: &IResult<&str,T>) {
    match *res {
        Ok((rest, ref value)) => {println!("Done {:?} {:?}", rest, value)},
        Err(ref err) => {println!("Error {:?}", err)},
    }
}


fn main() {
    let app = App::new("cli")
        .version("0.1.0")
        .author("Gero Posmyk-Leinemann")
        .about("Some cli test")
        .arg(Arg::with_name("n")
        .required(true)
        .takes_value(true)
        .index(1)
        .help("Number of tries"));
    let matches = app.get_matches();

    let n = value_t!(matches, "n", u32).unwrap_or(10);
    //let n = matches.value_of("n").unwrap();

    println!("Your number: {}", n);
}
