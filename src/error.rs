use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
/// Representation of any Error that can occur while parsing.
/// # Examples
///
/// ```compile_fail
///        assert_eq!(
///            evaluate_at(0.0, "5+%+4".to_string()),
///            Err(ParseError::UnknownSymbol),
///        );
///
///        assert_eq!(
///            evaluate_at(0.0, "5+3*".to_string()),
///            Err(ParseError::BadExpression)
///        );
///        assert_eq!(
///            evaluate_at(0.0, "2*(5))".to_string()),
///            Err(ParseError::UnbalancedParens)
///        );
///        assert_eq!(
///            evaluate_at(0.0, "5+3*".to_string()),
///            Err(ParseError::BadExpression)
///        );
/// ```
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
