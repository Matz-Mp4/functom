#[cfg(test)]
mod tests {

    use functom::evaluater::error::ParseError;
    use functom::evaluater::evaluate::evaluate_at;

    #[test]
    fn test_01() {
        assert_eq!(evaluate_at(3.0, "x+2+5*3".to_string()), Ok(20.0));
        assert_eq!(evaluate_at(0.0, "2^(3+5)".to_string()), Ok(256.0));
        assert_eq!(evaluate_at(0.0, "-5^2".to_string()), Ok(-25.0));
        assert_eq!(evaluate_at(0.0, "(-5)^2".to_string()), Ok(25.0));
        /* assert_eq!(evaluate_at(0.0,"4^0.5".to_string()), Ok(2.0)); */
    }

    #[test]
    fn test_02() {
        assert_eq!(evaluate_at(0.0, "sqrt((9))".to_string()), Ok(3.0));
        assert_eq!(evaluate_at(0.0, "ln(1)".to_string()), Ok(0.0));
        assert_eq!(evaluate_at(0.0, "sqrt(4*10-4)".to_string()), Ok(6.0));
        assert_eq!(
            evaluate_at(0.0, "(cos(ln(1)+sqrt(1) - 1))*2".to_string()),
            Ok(2.0)
        );
        assert_eq!(evaluate_at(0.0, "e^(ln(1))".to_string()), Ok(1.0));
        assert!(evaluate_at(0.0, "pi^2".to_string()).unwrap() < 10.0);
        assert_eq!(evaluate_at(0.0, "-sqrt(9) * 2".to_string()), Ok(-6.0));
    }

    #[test]
    fn test_03() {
        assert_eq!(
            evaluate_at(0.0, "5+%+4".to_string()),
            Err(ParseError::UnknownSymbol),
        );

        assert_eq!(
            evaluate_at(0.0, "5+3*".to_string()),
            Err(ParseError::BadExpression)
        );
        assert_eq!(
            evaluate_at(0.0, "2*(5))".to_string()),
            Err(ParseError::UnbalancedParens)
        );
        assert_eq!(
            evaluate_at(0.0, "5+3*".to_string()),
            Err(ParseError::BadExpression)
        );
    }
}
