
#[derive(Debug, PartialEq)]
pub enum Statements<'a> {
    List { list: std::vec::Vec<Statement<'a>> }
}

#[derive(Debug, PartialEq)]
pub enum Comment<'a> {
    Line { text: &'a str }
}

#[derive(Debug, PartialEq)]
pub enum Statement<'a> {
    Equivalence { lhs: Box<StatementLhs<'a>>, rhs: Box<StatementRhs<'a>> }
}

#[derive(Debug, PartialEq)]
pub enum StatementLhs<'a> {
    PatternName { name: &'a str }
}

#[derive(Debug, PartialEq)]
pub enum StatementRhs<'a> {
    Expr { expr: Box<SequentialExpression<'a>> }
    //ObjectPattern {}
}

#[derive(Debug, PartialEq)]
pub enum SequentialExpression<'a> {
    Variable { symbol: QuantifiedSymbol<'a> },
    Sequence { symbols: Vec<QuantifiedSymbol<'a>> },
    Optional { lhs: Box<SequentialExpression<'a>>, rhs: Box<SequentialExpression<'a>> },
}

#[derive(Debug, PartialEq)]
pub enum QuantifiedSymbol<'a> {
    QuantifiedSymbol { symbol: Symbol<'a>, quantifier: Quantifier }
}

#[derive(Debug, PartialEq)]
pub enum Symbol<'a> {
    Terminal { value: &'a str },
    //NonTerminal { },
}

#[derive(Debug, PartialEq, )]
pub enum Quantifier {
    One,
    OneOrMore,
    ZeroOrMore,
}
