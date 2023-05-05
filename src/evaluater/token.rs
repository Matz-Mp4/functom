use crate::evaluater::error::ParseError;
use std::collections::VecDeque;


#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    BinOp(char),
    Num(f64),
    UnOp(char),
    Bracket(char),
    Var(char),
    Const(char),
}

impl Token {
    pub fn tokenizer(expr: String) -> Result<VecDeque<Token>, ParseError> {
        let temp = Token::remove_whitespace(expr.as_str());
        let mut tokens = VecDeque::new();
        let chars = temp.chars();
        let mut parens = Vec::new();
        let mut buffer = String::new();
        let mut leading = true;

        for ch in chars {
            match ch {
                'x' => {
                    leading = false;
                    tokens.push_back(Token::Var('x'));
                }
                'a'..='z' | 'A'..='Z' => {
                    leading = false;
                    buffer.push(ch);

                    if buffer.len() < 5 {
                        match buffer.as_str() {
                            "sin" => {
                                tokens.push_back(Token::UnOp('s'));
                                buffer.clear();
                            }
                            "tan" => {
                                tokens.push_back(Token::UnOp('t'));
                                buffer.clear();
                            }
                            "sqrt" => {
                                tokens.push_back(Token::UnOp('r'));
                                buffer.clear();
                            }

                            "cos" => {
                                tokens.push_back(Token::UnOp('c'));
                                buffer.clear();
                            }
                            "ln" => {
                                tokens.push_back(Token::UnOp('l'));
                                buffer.clear();
                            }

                            "log" => {
                                tokens.push_back(Token::UnOp('k'));
                                buffer.clear();
                            }

                            "e" => {
                                tokens.push_back(Token::Const('e'));
                                buffer.clear();
                                leading = false;
                            }

                            "pi" => {
                                tokens.push_back(Token::Const('p'));
                                buffer.clear();

                                leading = false;
                            }
                            _ => {}
                        }
                    } else {
                        return Err(ParseError::UnknownSymbol);
                    }
                }
                '0'..='9' => {
                    match tokens.back_mut() {
                        Some(Token::Num(n)) => {
                            *n = *n * 10 as f64 + (ch as u32 - 48) as f64;
                        }
                        _ => {
                            let digit = ch as u32 - 48;
                            tokens.push_back(Token::Num(digit as f64));
                        }
                    }
                    leading = false;
                }

                '(' => {
                    tokens.push_back(Token::Bracket('('));
                    parens.push('(');
                    leading = true;
                }

                ')' => {
                    leading = false;
                    tokens.push_back(Token::Bracket(')'));
                    if let Some(p) = parens.pop() {
                        if p != '(' {
                            return Err(ParseError::UnbalancedParens);
                        }
                    } else {
                        return Err(ParseError::UnbalancedParens);
                    }
                }
                '+' => {
                    tokens.push_back(Token::BinOp('+'));
                    leading = true;
                }
                '-' => {
                    if leading {
                        tokens.push_back(Token::UnOp('~'));
                    } else {
                        tokens.push_back(Token::BinOp('-'));
                    }
                }
                '*' => {
                    leading = true;
                    tokens.push_back(Token::BinOp('*'));
                }
                '/' => {
                    leading = true;
                    tokens.push_back(Token::BinOp('/'));
                }
                '^' => {
                    leading = true;
                    tokens.push_back(Token::BinOp('^'));
                }
                '\n' => {}

                _ => {
                    return Err(ParseError::UnknownSymbol);
                }
            }
        }

        if parens.len() > 0 {
            return Err(ParseError::UnbalancedParens);
        }

        Ok(tokens)
    }

    fn remove_whitespace(s: &str) -> String {
        s.split_whitespace().collect()
    }
}
