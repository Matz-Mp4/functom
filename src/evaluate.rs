use crate::{error::*,  parser, token::Token};
use std::{collections::VecDeque, f64::consts};

pub fn evaluate_at(value: f64, function: String) -> Result<f64, ParseError> {
    let tokens = Token::tokenizer(function)?;
    let posfix = parser::infix_to_posfix(tokens)?;
    calculate_at(value, posfix)
}

pub fn evaluate_on_range(
    begin: f64,
    end: f64,
    function: String,
    inc: f64,
) -> Result<Vec<f64>, ParseError> {

    let mut func_values: Vec<f64> = Vec::new();
    let tokens = Token::tokenizer(function)?;
    let posfix = parser::infix_to_posfix(tokens)?;
    let mut temp = begin;

    while temp <= end {
        func_values.push(calculate_at(temp, posfix.clone())?);
        temp +=  inc;
    }

    Ok(func_values)
}

/// Calculate a tokenized RPN-notated Expression
pub fn calculate_at(value: f64, toks: VecDeque<Token>) -> Result<f64, ParseError> {
    let mut stack: Vec<f64> = Vec::new();

    for tok in toks {
        match tok {
            // Number
            Token::Num(num) => stack.push(num),
            Token::Var('x') => stack.push(value),

            Token::Const('e') => stack.push(consts::E),
            Token::Const('p') => stack.push(consts::PI),

            // Binary operation (like '+', '-', '*', '/')
            Token::BinOp(op) => {
                if let (Some(a), Some(b)) = (stack.pop(), stack.pop()) {
                    stack.push(match op {
                        '+' => b + a,
                        '-' => b - a,
                        '*' => b * a,
                        '/' => b / a,
                        '^' => b.powf(a),
                        _ => return Err(ParseError::UnknownSymbol),
                    });
                } else {
                    return Err(ParseError::BadExpression);
                }
            }

            // Unary operation (like '~')
            Token::UnOp(op) => {
                if let Some(v) = stack.pop() {
                    if op == '~' {
                        stack.push(-v);
                    } else {
                        stack.push(match op {
                            's' => v.sin(),
                            'c' => v.cos(),
                            't' => v.tan(),
                            'r' => v.sqrt(),
                            'l' => v.ln(),
                            'k' => v.log10(),
                            _ => return Err(ParseError::BadExpression),
                        });
                    }
                } else {
                    return Err(ParseError::BadExpression);
                }
            }

            _ => {}
        }
    }

    let len = stack.len();
    if len == 0 {
        Err(ParseError::BadExpression)
    } else if len == 1 {
        Ok(stack.pop().unwrap())
    } else {
        Err(ParseError::BadExpression)
    }
}
