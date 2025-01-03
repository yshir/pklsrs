#[derive(Debug)]
pub enum Token {
    Number(i32),
    Ident(String),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen,
    Eq,
    Ne,
    Lt,
    Lte,
    Gt,
    Gte,
    Assign,
    Eof,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut chars = input.chars().peekable();
    let mut tokens: Vec<Token> = vec![];

    while let Some(c) = chars.next() {
        match c {
            ' ' | '\t' | '\n' => {}
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Asterisk),
            '/' => tokens.push(Token::Slash),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '<' => match chars.peek() {
                Some('=') => {
                    chars.next();
                    tokens.push(Token::Lte);
                }
                _ => tokens.push(Token::Lt),
            },
            '>' => match chars.peek() {
                Some('=') => {
                    chars.next();
                    tokens.push(Token::Gte);
                }
                _ => tokens.push(Token::Gt),
            },
            '=' => match chars.peek() {
                Some('=') => {
                    chars.next();
                    tokens.push(Token::Eq);
                }
                _ => tokens.push(Token::Assign),
            },
            '!' => {
                let c = chars.next().unwrap();
                match c {
                    '=' => tokens.push(Token::Ne),
                    _ => panic!("unexpected character: {}", c),
                }
            }
            '0'..='9' => {
                let mut num = c.to_string();
                while let Some(c) = chars.peek() {
                    match c {
                        '0'..='9' => num.push(chars.next().unwrap()),
                        _ => break,
                    }
                }
                tokens.push(Token::Number(num.parse().unwrap()))
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut str = c.to_string();
                while let Some(c) = chars.peek() {
                    match c {
                        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => str.push(chars.next().unwrap()),
                        _ => break,
                    }
                }
                tokens.push(Token::Ident(str))
            }
            _ => panic!("unexpected character: {}", c),
        }
    }

    tokens.push(Token::Eof);
    tokens
}
