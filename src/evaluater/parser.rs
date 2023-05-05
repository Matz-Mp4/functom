use std::collections::VecDeque;

use crate::evaluater::{error::ParseError, token::Token};

fn precedency(tok: &Token) -> u8 {
    match tok {
        Token::UnOp('s')
        | Token::UnOp('c')
        | Token::UnOp('t')
        | Token::UnOp('l')
        | Token::BinOp('^')
        | Token::UnOp('~')
        | Token::UnOp('k')
        | Token::UnOp('r') => 6,

        Token::BinOp('+') | Token::BinOp('-') => 3,
        Token::BinOp('/') | Token::BinOp('*') => 4,

        Token::Const(_) => 0,

        _ => 0,
    }
}

pub fn valid_infix(tokens: &VecDeque<Token>) -> Result<&VecDeque<Token>, ParseError> {
    let iter = tokens.into_iter();
    let mut last: Option<&Token> = None;

    for token in iter {
        match *token {
            Token::Num(_) | Token::Var('x') | Token::Const(_) => {
                if let Some(t) = last {
                    match *t {
                        Token::Num(_) | Token::Var('x') | Token::Bracket(')') => {
                            return Err(ParseError::OperatorExpected);
                        }
                        _ => {}
                    }
                }
            }

            Token::Bracket('(') | Token::UnOp(_) => {
                if let Some(t) = last {
                    match *t {
                        Token::Num(_) | Token::Var('x') | Token::Const(_) => {
                            return Err(ParseError::OperatorExpected);
                        }
                        _ => {}
                    }
                }
            }

            Token::Bracket(')') => {
                if let Some(t) = last {
                    match *t {
                        Token::BinOp(_) | Token::UnOp(_) => {
                            return Err(ParseError::BadExpression);
                        }
                        _ => {}
                    }
                }
            }
            Token::BinOp(_) => {
                if let Some(t) = last {
                    match *t {
                        Token::BinOp(_) | Token::Bracket('(') | Token::UnOp(_) => {
                            return Err(ParseError::BadExpression);
                        }
                        _ => {}
                    }
                }
            }

            _ => {}
        }

        last = Some(token);
    }

    Ok(tokens)
}

pub fn infix_to_posfix(tokens: VecDeque<Token>) -> Result<VecDeque<Token>, ParseError> {
    let mut posfix = VecDeque::new();
    let mut delayed_operations = Vec::new();
    let mut prior: &Token;

    if let Err(t) = valid_infix(&tokens) {
        return Err(t);
    }

    for token in tokens {
        match token {
            Token::Num(_) | Token::Var('x') => posfix.push_back(token),

            Token::Const(_) => posfix.push_back(token),

            Token::Bracket('(') => {
                delayed_operations.push(token);
            }

            Token::Bracket(')') => {
                prior = delayed_operations.last_mut().unwrap();

                while *prior != Token::Bracket('(') {
                    posfix.push_back(prior.clone());
                    delayed_operations.pop();
                    prior = delayed_operations.last().unwrap();
                }

                delayed_operations.pop();
            }

            Token::UnOp(_) | Token::BinOp(_) => {
                let mut end_right = false;

                loop {
                    if delayed_operations.is_empty() {
                        end_right = true;
                    } else {
                        prior = delayed_operations.last().unwrap();

                        if *prior == Token::Bracket('(') {
                            end_right = true;
                        } else if precedency(prior) < precedency(&token) {
                            end_right = true;
                        } else if precedency(&token) == 6 {
                            end_right = true;
                        } else {
                            posfix.push_back(prior.clone());
                        }
                        if !end_right {
                            delayed_operations.pop();
                        }
                    }
                    if end_right {
                        break;
                    }
                }
                delayed_operations.push(token.clone());
            }
            _ => {}
        }
    }

    while !delayed_operations.is_empty() {
        prior = delayed_operations.last_mut().unwrap();
        posfix.push_back(prior.clone());
        delayed_operations.pop();
    }

    Ok(posfix)
}
