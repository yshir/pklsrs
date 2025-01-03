use core::slice::Iter;
use std::iter::Peekable;

use crate::tokenizer::Token;

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Neg(Box<Expr>),
    Add { lhs: Box<Expr>, rhs: Box<Expr> },
    Sub { lhs: Box<Expr>, rhs: Box<Expr> },
    Mul { lhs: Box<Expr>, rhs: Box<Expr> },
    Div { lhs: Box<Expr>, rhs: Box<Expr> },
    Eq { lhs: Box<Expr>, rhs: Box<Expr> },
    Ne { lhs: Box<Expr>, rhs: Box<Expr> },
    Lt { lhs: Box<Expr>, rhs: Box<Expr> },
    Lte { lhs: Box<Expr>, rhs: Box<Expr> },
}

pub fn parse(tokens: Vec<Token>) -> Expr {
    let mut tokens = tokens.iter().peekable();
    parse_expr(&mut tokens)
}

/// expr := equal
fn parse_expr(tokens: &mut Peekable<Iter<'_, Token>>) -> Expr {
    parse_equal(tokens)
}

/// equal := compare ('=='|'!=' compare)*
fn parse_equal(tokens: &mut Peekable<Iter<'_, Token>>) -> Expr {
    let mut expr = parse_compare(tokens);

    while let Some(token) = tokens.peek() {
        match token {
            Token::Eq => {
                tokens.next();
                expr = Expr::Eq {
                    lhs: Box::new(expr),
                    rhs: Box::new(parse_compare(tokens)),
                }
            }
            Token::Ne => {
                tokens.next();
                expr = Expr::Ne {
                    lhs: Box::new(expr),
                    rhs: Box::new(parse_compare(tokens)),
                }
            }
            _ => break,
        }
    }

    expr
}

/// compare := add ('<'|'<='|'>'|'>=' add)*
fn parse_compare(tokens: &mut Peekable<Iter<'_, Token>>) -> Expr {
    let mut expr = parse_add(tokens);

    while let Some(token) = tokens.peek() {
        match token {
            Token::Lt => {
                tokens.next();
                expr = Expr::Lt {
                    lhs: Box::new(expr),
                    rhs: Box::new(parse_add(tokens)),
                }
            }
            Token::Lte => {
                tokens.next();
                expr = Expr::Lte {
                    lhs: Box::new(expr),
                    rhs: Box::new(parse_add(tokens)),
                }
            }
            Token::Gt => {
                tokens.next();
                expr = Expr::Lt {
                    lhs: Box::new(parse_add(tokens)),
                    rhs: Box::new(expr),
                }
            }
            Token::Gte => {
                tokens.next();
                expr = Expr::Lte {
                    lhs: Box::new(parse_add(tokens)),
                    rhs: Box::new(expr),
                }
            }
            _ => break,
        }
    }

    expr
}

/// add := mul ('+'|'-' mul)*
fn parse_add(tokens: &mut Peekable<Iter<'_, Token>>) -> Expr {
    let mut expr = parse_mul(tokens);

    while let Some(token) = tokens.peek() {
        match token {
            Token::Plus => {
                tokens.next();
                expr = Expr::Add {
                    lhs: Box::new(expr),
                    rhs: Box::new(parse_mul(tokens)),
                }
            }
            Token::Minus => {
                tokens.next();
                expr = Expr::Sub {
                    lhs: Box::new(expr),
                    rhs: Box::new(parse_mul(tokens)),
                }
            }
            _ => break,
        };
    }

    expr
}

/// mul := unary ('*'|'/' unary)*
fn parse_mul(tokens: &mut Peekable<Iter<'_, Token>>) -> Expr {
    let mut expr = parse_unary(tokens);

    while let Some(token) = tokens.peek() {
        match token {
            Token::Asterisk => {
                tokens.next();
                expr = Expr::Mul {
                    lhs: Box::new(expr),
                    rhs: Box::new(parse_unary(tokens)),
                }
            }
            Token::Slash => {
                tokens.next();
                expr = Expr::Div {
                    lhs: Box::new(expr),
                    rhs: Box::new(parse_unary(tokens)),
                }
            }
            _ => break,
        };
    }

    expr
}

/// unary := ('-'|'+')? primary
fn parse_unary(tokens: &mut Peekable<Iter<'_, Token>>) -> Expr {
    let token = tokens.peek().unwrap();
    match token {
        Token::Minus => {
            tokens.next();
            Expr::Neg(Box::new(parse_primary(tokens)))
        }
        Token::Plus => {
            tokens.next();
            parse_primary(tokens)
        }
        _ => parse_primary(tokens),
    }
}

/// primary := number | '(' expr ')'
fn parse_primary(tokens: &mut Peekable<Iter<'_, Token>>) -> Expr {
    let token = tokens.peek().unwrap();
    match token {
        Token::LParen => {
            tokens.next();
            let expr = parse_expr(tokens);
            let token = tokens.next().unwrap();
            match token {
                Token::RParen => expr,
                _ => panic!("unexpected token: {:?}", token),
            }
        }
        Token::Number(_) => parse_number(tokens),
        _ => panic!("unexpected token: {:?}", token),
    }
}

/// number := [0-9]+
fn parse_number(tokens: &mut Peekable<Iter<'_, Token>>) -> Expr {
    let token = tokens.next().unwrap();

    match token {
        Token::Number(n) => Expr::Number(*n),
        _ => panic!("unexpected token: {:?}", token),
    }
}
