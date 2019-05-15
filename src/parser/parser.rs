use std::fmt::Debug;

use nom;
use nom::types::CompleteStr;

use ast::{
    QuantifiedSymbol, Quantifier, Statement, StatementLhs, StatementLhs::PatternName, StatementRhs,
    Statements, Symbol, SequentialExpression, Comment,
};

pub fn to_str(s: &str) -> CompleteStr {
    CompleteStr::from(s)
}

named!(pub statement_list<CompleteStr, Statements>,
    do_parse!(
        //list: many1!(ws!(statement))    >>
        list: fold_many1!( alt!( ws!(statement) ), Vec::new(), |mut acc: Vec<Statement>, item| {
            match item {
                Statement::Equivalence {lhs, rhs} => acc.push(item),
            }
            acc
        })   >>
        (Statements::List {
            list
        })
    )
);



#[test]
fn parse_statement_list() {
    let result = statement_list(to_str("//URI = [a b c].\n// comment //\n   // test comment\nURI = [a b c].\n"));
    dump(&result);
    assert_eq!(
        result,
        Ok((
            to_str(""),
            Statements::List {
                list: vec![
                    Statement::Equivalence {
                        lhs: Box::new(PatternName { name: "URI" }),
                        rhs: Box::new(StatementRhs::Expr {
                            expr: Box::new(SequentialExpression::Sequence {
                                symbols: vec![
                                    QuantifiedSymbol::QuantifiedSymbol {
                                        symbol: Symbol::Terminal { value: "a" },
                                        quantifier: Quantifier::One
                                    },
                                    QuantifiedSymbol::QuantifiedSymbol {
                                        symbol: Symbol::Terminal { value: "b" },
                                        quantifier: Quantifier::One
                                    },
                                    QuantifiedSymbol::QuantifiedSymbol {
                                        symbol: Symbol::Terminal { value: "c" },
                                        quantifier: Quantifier::One
                                    }
                                ]
                            })
                        })
                    },
                ]
            }
        ))
    );
}

#[test]
fn parse_statement_list() {
    let result = statement_list(to_str("URI = [a b c].\nURI = [a b c].\n"));
    dump(&result);
    assert_eq!(
        result,
        Ok((
            to_str(""),
            Statements::List {
                list: vec![
                    Statement::Equivalence {
                        lhs: Box::new(PatternName { name: "URI" }),
                        rhs: Box::new(StatementRhs::Expr {
                            expr: Box::new(SequentialExpression::Sequence {
                                symbols: vec![
                                    QuantifiedSymbol::QuantifiedSymbol {
                                        symbol: Symbol::Terminal { value: "a" },
                                        quantifier: Quantifier::One
                                    },
                                    QuantifiedSymbol::QuantifiedSymbol {
                                        symbol: Symbol::Terminal { value: "b" },
                                        quantifier: Quantifier::One
                                    },
                                    QuantifiedSymbol::QuantifiedSymbol {
                                        symbol: Symbol::Terminal { value: "c" },
                                        quantifier: Quantifier::One
                                    }
                                ]
                            })
                        })
                    },
                    Statement::Equivalence {
                        lhs: Box::new(PatternName { name: "URI" }),
                        rhs: Box::new(StatementRhs::Expr {
                            expr: Box::new(SequentialExpression::Sequence {
                                symbols: vec![
                                    QuantifiedSymbol::QuantifiedSymbol {
                                        symbol: Symbol::Terminal { value: "a" },
                                        quantifier: Quantifier::One
                                    },
                                    QuantifiedSymbol::QuantifiedSymbol {
                                        symbol: Symbol::Terminal { value: "b" },
                                        quantifier: Quantifier::One
                                    },
                                    QuantifiedSymbol::QuantifiedSymbol {
                                        symbol: Symbol::Terminal { value: "c" },
                                        quantifier: Quantifier::One
                                    }
                                ]
                            })
                        })
                    }
                ]
            }
        ))
    );
}

fn is_line_ending(c: u8) -> bool {
    true
}

named!(comment<CompleteStr, Comment>,
    do_parse!(
        many0!(nom::space)  >>
        tag_s!("//")    >>
        text: take_while!(is_line_ending) >>
        (Comment::Line {
            text: &text
        })
    )
);

#[test]
fn parse_comment() {
    let result = comment(to_str("//test"));
    dump(&result);
    assert_eq!(
        result,
        Ok((
            to_str(""),
            Comment::Line {
                text: "test"
            }
        ))
    );
}

named!(pub statement<CompleteStr, Statement>,
    do_parse!(
        lhs: statement_lhs >>
            ws!( tag_s!("=") ) >>
        rhs: statement_rhs >>
            opt!(many1!(nom::space)) >>
            tag_s!(".") >>
        (Statement::Equivalence {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs)
        })
    )
);

#[test]
fn parse_statement_equivalence_sequential() {
    let result = statement(to_str("URI = [a b c]."));
    dump(&result);
    assert_eq!(
        result,
        Ok((
            to_str(""),
            Statement::Equivalence {
                lhs: Box::new(PatternName { name: "URI" }),
                rhs: Box::new(StatementRhs::Expr {
                    expr: Box::new(SequentialExpression::Sequence {
                        symbols: vec![
                            QuantifiedSymbol::QuantifiedSymbol {
                                symbol: Symbol::Terminal { value: "a" },
                                quantifier: Quantifier::One
                            },
                            QuantifiedSymbol::QuantifiedSymbol {
                                symbol: Symbol::Terminal { value: "b" },
                                quantifier: Quantifier::One
                            },
                            QuantifiedSymbol::QuantifiedSymbol {
                                symbol: Symbol::Terminal { value: "c" },
                                quantifier: Quantifier::One
                            }
                        ]
                    })
                })
            }
        ))
    );
}

#[test]
fn parse_statement_equivalence_optional() {
    let result = statement(to_str("Bit = (a | b)."));
    dump(&result);
    assert_eq!(
        result,
        Ok((
            to_str(""),
            Statement::Equivalence {
                lhs: Box::new(PatternName { name: "Bit" }),
                rhs: Box::new(StatementRhs::Expr {
                    expr: Box::new(SequentialExpression::Optional {
                        lhs: Box::new(SequentialExpression::Variable {
                            symbol: QuantifiedSymbol::QuantifiedSymbol {
                                symbol: Symbol::Terminal { value: "a" },
                                quantifier: Quantifier::One
                            }
                        }),
                        rhs: Box::new(SequentialExpression::Variable {
                            symbol: QuantifiedSymbol::QuantifiedSymbol {
                                symbol: Symbol::Terminal { value: "b" },
                                quantifier: Quantifier::One
                            }
                        }),
                    })
                })
            }
        ))
    );
}

named!(statement_lhs<CompleteStr, StatementLhs>,
    do_parse!(
        name: pattern_name >>
        (PatternName { name: &name })
    )
);
named!(pattern_name<CompleteStr, CompleteStr>, take_while_s!( is_alpha ));

#[test]
fn parse_pattern_name_simple() {
    let result = statement_lhs(to_str("URI "));
    dump(&result);
    assert_eq!(result, Ok((to_str(" "), PatternName { name: "URI" })));
}

named!(statement_rhs<CompleteStr, StatementRhs>,
    do_parse!(
        expr: expr     >>
        (StatementRhs::Expr {
            expr: Box::new(expr)
        })
    )
);

#[test]
fn parse_statement_rhs_sequence() {
    let result = statement_rhs(to_str("[a b c]"));
    dump(&result);
    assert_eq!(
        result,
        Ok((
            to_str(""),
            StatementRhs::Expr {
                expr: Box::new(SequentialExpression::Sequence {
                    symbols: vec![
                        QuantifiedSymbol::QuantifiedSymbol {
                            symbol: Symbol::Terminal { value: "a" },
                            quantifier: Quantifier::One
                        },
                        QuantifiedSymbol::QuantifiedSymbol {
                            symbol: Symbol::Terminal { value: "b" },
                            quantifier: Quantifier::One
                        },
                        QuantifiedSymbol::QuantifiedSymbol {
                            symbol: Symbol::Terminal { value: "c" },
                            quantifier: Quantifier::One
                        }
                    ]
                })
            }
        ))
    );
}

named!(expr<CompleteStr, SequentialExpression>,
    alt!(expr_sequence | expr_variable | expr_optional)
);

#[test]
fn parse_expr_opt() {
    let result = expr(to_str("(a | b)"));
    assert_eq!(
        result,
        Ok((
            to_str(""),
            SequentialExpression::Optional {
                lhs: Box::new(SequentialExpression::Variable {
                    symbol: QuantifiedSymbol::QuantifiedSymbol {
                        symbol: Symbol::Terminal { value: "a" },
                        quantifier: Quantifier::One
                    }
                }),
                rhs: Box::new(SequentialExpression::Variable {
                    symbol: QuantifiedSymbol::QuantifiedSymbol {
                        symbol: Symbol::Terminal { value: "b" },
                        quantifier: Quantifier::One
                    }
                }),
            }
        ))
    );
}

#[test]
fn parse_expr_seq() {
    let result = expr(to_str("[a b c]"));
    assert_eq!(
        result,
        Ok((
            to_str(""),
            SequentialExpression::Sequence {
                symbols: vec![
                    QuantifiedSymbol::QuantifiedSymbol {
                        symbol: Symbol::Terminal { value: "a" },
                        quantifier: Quantifier::One
                    },
                    QuantifiedSymbol::QuantifiedSymbol {
                        symbol: Symbol::Terminal { value: "b" },
                        quantifier: Quantifier::One
                    },
                    QuantifiedSymbol::QuantifiedSymbol {
                        symbol: Symbol::Terminal { value: "c" },
                        quantifier: Quantifier::One
                    }
                ]
            }
        ))
    );
}

named!(expr_optional<CompleteStr, SequentialExpression>,
    do_parse!(
        tag_s!("(")     >>
        lhs: ws!(expr)  >>
        tag_s!("|")     >>
        rhs: ws!(expr)  >>
        tag_s!(")")     >>
        (SequentialExpression::Optional {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    )
);

#[test]
fn parse_optional_expression() {
    let result = expr_optional(to_str("(a | b)"));
    dump(&result);
    assert_eq!(
        result,
        Ok((
            to_str(""),
            SequentialExpression::Optional {
                lhs: Box::new(SequentialExpression::Variable {
                    symbol: QuantifiedSymbol::QuantifiedSymbol {
                        symbol: Symbol::Terminal { value: "a" },
                        quantifier: Quantifier::One
                    }
                }),
                rhs: Box::new(SequentialExpression::Variable {
                    symbol: QuantifiedSymbol::QuantifiedSymbol {
                        symbol: Symbol::Terminal { value: "b" },
                        quantifier: Quantifier::One
                    }
                }),
            }
        ))
    );
}

named!(expr_sequence<CompleteStr, SequentialExpression>,
    do_parse!(
        tag_s!("[") >>
        opt!(many1!(nom::space)) >>
        symbols: separated_list!(nom::space, quantified_symbol) >>
        opt!(many1!(nom::space)) >>
        tag_s!("]") >>
        (SequentialExpression::Sequence {
            symbols
        })
    )
);

#[test]
fn parse_sequential_expression() {
    let result = expr_sequence(to_str("[a b c]"));
    dump(&result);
    assert_eq!(
        result,
        Ok((
            to_str(""),
            SequentialExpression::Sequence {
                symbols: vec![
                    QuantifiedSymbol::QuantifiedSymbol {
                        symbol: Symbol::Terminal { value: "a" },
                        quantifier: Quantifier::One
                    },
                    QuantifiedSymbol::QuantifiedSymbol {
                        symbol: Symbol::Terminal { value: "b" },
                        quantifier: Quantifier::One
                    },
                    QuantifiedSymbol::QuantifiedSymbol {
                        symbol: Symbol::Terminal { value: "c" },
                        quantifier: Quantifier::One
                    }
                ]
            }
        ))
    );
}

named!(expr_variable<CompleteStr, SequentialExpression>,
    do_parse!(
        symbol: quantified_symbol       >>
        (SequentialExpression::Variable {
            symbol
        })
    )
);

#[test]
fn parse_variable() {
    let result = expr_variable(to_str("a"));
    dump(&result);
    assert_eq!(
        result,
        Ok((
            to_str(""),
            SequentialExpression::Variable {
                symbol: QuantifiedSymbol::QuantifiedSymbol {
                    symbol: Symbol::Terminal { value: "a" },
                    quantifier: Quantifier::One
                },
            }
        ))
    );
}

named!(quantified_symbol<CompleteStr, QuantifiedSymbol>,
    do_parse!(
        symbol: symbol >>
        quantifier: quantifier >>
        (QuantifiedSymbol::QuantifiedSymbol { symbol, quantifier })
    )
);

#[test]
fn parse_quantified_symbol_one() {
    let result = quantified_symbol(to_str("a "));
    dump(&result);
    assert_eq!(
        result,
        Ok((
            to_str(" "),
            QuantifiedSymbol::QuantifiedSymbol {
                symbol: Symbol::Terminal { value: "a" },
                quantifier: Quantifier::One
            }
        ))
    );
}

named!(quantifier<CompleteStr, Quantifier>,
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

#[test]
fn parse_quantifier_one_or_more() {
    let result = quantifier(to_str("+"));
    dump(&result);
    assert_eq!(result, Ok((to_str(""), Quantifier::OneOrMore)));
}

named!(symbol<CompleteStr, Symbol>,
    do_parse!(
        value: symbol_value >>
        (Symbol::Terminal { value: &value })
    )
);

fn is_alpha(chr: char) -> bool {
    nom::is_alphabetic(chr as u8)
}
named!(symbol_value<CompleteStr, CompleteStr>, take_while1_s!( is_alpha ));

#[test]
fn parse_terminal() {
    let result = symbol(to_str("a "));
    dump(&result);
    assert_eq!(result, Ok((to_str(" "), Symbol::Terminal { value: "a" })));
}

pub fn dump<T: Debug>(res: &nom::IResult<CompleteStr, T>) {
    match *res {
        Ok((rest, ref value)) => println!("Done {:?} {:?}", rest, value),
        Err(ref err) => println!("Error {:?}", err),
    }
}
