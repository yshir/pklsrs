use core::slice::Iter;
use std::iter::Peekable;

use crate::tokenizer::Token;

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Add { lhs: Box<Expr>, rhs: Box<Expr> },
    Sub { lhs: Box<Expr>, rhs: Box<Expr> },
}

pub fn parse(tokens: Vec<Token>) -> Expr {
    let mut tokens = tokens.iter().peekable();
    parse_expr(&mut tokens)
}

/// expr := number ('+'|'-' number)*
fn parse_expr(tokens: &mut Peekable<Iter<'_, Token>>) -> Expr {
    let mut expr = parse_number(tokens);

    loop {
        let token = tokens.next().unwrap();
        match token {
            Token::Plus => {
                expr = Expr::Add {
                    lhs: Box::new(expr),
                    rhs: Box::new(parse_number(tokens)),
                }
            }
            Token::Minus => {
                expr = Expr::Sub {
                    lhs: Box::new(expr),
                    rhs: Box::new(parse_number(tokens)),
                }
            }
            Token::Eof => break,
            _ => panic!("unexpected token: {:?}", token),
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
