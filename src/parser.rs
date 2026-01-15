use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(i64),
    StringLit(String),
    Bool(bool),
    Variable(String),
    
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    
    Equals(Box<Expr>, Box<Expr>),
    NotEquals(Box<Expr>, Box<Expr>),
    LessThan(Box<Expr>, Box<Expr>),
    GreaterThan(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    VarDecl { name: String, value: Expr },
    Print(Expr),
}

fn parse_statement(tokens: &[Token], i: &mut usize) -> Statement {
    match &tokens[*i] {
        Token::V => {
            // parse variable declaration
            // v x = 5!
        }
        Token::P => {
            // parse print statement
            // p(x)!
        }
        _ => panic!("Unexpected token: {:?}", tokens[*i])
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<Statement> {
    let mut statements = Vec::new();
    let mut i = 0;
    
    while i < tokens.len() {
        // Parse one statement at a time
        // i will move forward as we consume tokens
    }
    
    statements
}