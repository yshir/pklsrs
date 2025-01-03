use core::slice::Iter;
use std::iter::Peekable;

use crate::tokenizer::Token;

#[derive(Debug)]
pub enum Expr {
    Number(i32),
}

pub fn parse(tokens: Vec<Token>) -> Expr {
    let mut tokens = tokens.iter().peekable();
    parse_expr(&mut tokens)
}

fn parse_expr(tokens: &mut Peekable<Iter<'_, Token>>) -> Expr {
    let token = tokens.next().unwrap();

    match token {
        Token::Number(n) => Expr::Number(*n),
        _ => panic!("unexpected token: {:?}", token),
    }
}
