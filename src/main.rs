use functom::evaluate::evaluate_at;
use functom::error::ParseError;
//TODO Comment Functions 
fn main() {

    assert_eq!(evaluate_at(0.0, "5+%+4".to_string()), Err(ParseError::UnknownSymbol));
}
