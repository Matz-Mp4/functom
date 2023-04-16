use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParseError {
    UnbalancedParens,
    OperatorExpected,
    BadExpression,
    UnknownSymbol,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {:?}", self)
    }
}
