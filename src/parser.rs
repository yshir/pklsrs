use core::slice::Iter;
use std::iter::Peekable;

use crate::tokenizer::Token;

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Add { lhs: Box<Expr>, rhs: Box<Expr> },
    Sub { lhs: Box<Expr>, rhs: Box<Expr> },
    Mul { lhs: Box<Expr>, rhs: Box<Expr> },
    Div { lhs: Box<Expr>, rhs: Box<Expr> },
}

pub fn parse(tokens: Vec<Token>) -> Expr {
    let mut tokens = tokens.iter().peekable();
    parse_expr(&mut tokens)
}

/// expr := add
fn parse_expr(tokens: &mut Peekable<Iter<'_, Token>>) -> Expr {
    parse_add(tokens)
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

/// mul := number ('*'|'/' number)*
fn parse_mul(tokens: &mut Peekable<Iter<'_, Token>>) -> Expr {
    let mut expr = parse_number(tokens);

    while let Some(token) = tokens.peek() {
        match token {
            Token::Asterisk => {
                tokens.next();
                expr = Expr::Mul {
                    lhs: Box::new(expr),
                    rhs: Box::new(parse_number(tokens)),
                }
            }
            Token::Slash => {
                tokens.next();
                expr = Expr::Div {
                    lhs: Box::new(expr),
                    rhs: Box::new(parse_number(tokens)),
                }
            }
            _ => break,
        };
    }

    expr
}

/// number := [0-9]+
fn parse_number(tokens: &mut Peekable<Iter<'_, Token>>) -> Expr {
    let token = tokens.next().unwrap();

    match token {
        Token::Number(n) => Expr::Number(*n),
        _ => panic!("unexpected token: {:?}", token),
    }
}
