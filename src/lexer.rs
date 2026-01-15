#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i64),
    StringLiteral(String),
    Bool(bool),
    Identifier(String),
    Plus,
    Minus,
    Multiply,
    Divide,
    Equals,
    EqualsEquals,
    NotEquals,
    LessThan,
    GreaterThan,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Bang,
    Comma,
    V,
    F,
    I,
    E,
    W,
    R,
    P
}

fn process_key(key: &str, tokens: &mut Vec<Token>) {
    if key.is_empty() {
        return;
    }
    
    match key {
        "true" => tokens.push(Token::Bool(true)),
        "false" => tokens.push(Token::Bool(false)),
        "v" => tokens.push(Token::V),
        "f" => tokens.push(Token::F),
        "i" => tokens.push(Token::I),
        "e" => tokens.push(Token::E),
        "w" => tokens.push(Token::W),
        "r" => tokens.push(Token::R),
        "p" => tokens.push(Token::P),
        _ => {
            if let Ok(num) = key.parse::<i64>() {
                tokens.push(Token::Number(num));
            } else {
                tokens.push(Token::Identifier(key.to_string()));
            }
        }
    }
}

pub fn lex(str: &str) -> Vec<Token> {
    let mut key = String::new();
    let mut tokens = Vec::new();
    let mut in_string = false;
    for _c in str.chars() {
        if in_string {
            if _c == '"' {
                in_string = false;
                tokens.push(Token::StringLiteral(key.clone()));
                key.clear();
            } else {
                key.push(_c);
            }
            continue;
        }
        match _c {
            '+' => {
                process_key(&key, &mut tokens);
                key.clear();
                tokens.push(Token::Plus);
            },
            '-' => {
                process_key(&key, &mut tokens);
                key.clear();
                tokens.push(Token::Minus);
            },
            '*' => {
                process_key(&key, &mut tokens);
                key.clear();
                tokens.push(Token::Multiply);
            },
            '/' => {
                process_key(&key, &mut tokens);
                key.clear();
                tokens.push(Token::Divide);
            },
            '=' => {
                process_key(&key, &mut tokens);
                key.clear();
                tokens.push(Token::Equals);
            },
            '@' => {
                process_key(&key, &mut tokens);
                key.clear();
                tokens.push(Token::EqualsEquals);
            },
            '#' => {
                process_key(&key, &mut tokens);
                key.clear();
                tokens.push(Token::NotEquals);
            },
            '!' => {
                process_key(&key, &mut tokens);
                key.clear();
                tokens.push(Token::Bang);
            },
            '<' => {
                process_key(&key, &mut tokens);
                key.clear();
                tokens.push(Token::LessThan);
            },
            '>' => {
                process_key(&key, &mut tokens);
                key.clear();
                tokens.push(Token::GreaterThan);
            },
            '"' => {
                process_key(&key, &mut tokens);
                key.clear();
                in_string = true;
            },
            '(' => {
                process_key(&key, &mut tokens);
                key.clear();
                tokens.push(Token::LParen);
            },
            ')' => {
                process_key(&key, &mut tokens);
                key.clear();
                tokens.push(Token::RParen);
            },
            '{' => {
                process_key(&key, &mut tokens);
                key.clear();
                tokens.push(Token::LBrace);
            },
            '}' => {
                process_key(&key, &mut tokens);
                key.clear();
                tokens.push(Token::RBrace);
            },
            ',' => {
                process_key(&key, &mut tokens);
                key.clear();
                tokens.push(Token::Comma);
            },
            _ => {
                if _c.is_whitespace() {
                    process_key(&key, &mut tokens);
                    key.clear();
                }
                else {
                    key.push(_c);
                }
            }
        } 
    }
    process_key(&key, &mut tokens);
    tokens
}