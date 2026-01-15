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
            *i += 1;
            let var = if let Token::Identifier(name) = &tokens[*i] {
                name.clone()
            } else {
                panic!("Expected identifier after 'v', got {:?}", tokens[*i+1]);
            };
            *i += 1;
            if tokens[*i] != Token::Equals {
                panic!("Expected '=' after variable name, got {:?}", tokens[*i]);
            }
            *i += 1;
            let value = parse_expr(tokens, i);
            if tokens[*i] != Token::Bang {
                panic!("Expected '!' at end of variable declaration, got {:?}", tokens[*i]);
            }
            *i += 1;
            
            Statement::VarDecl { name: var, value }
        }
        Token::P => {
            *i += 1; // consume 'p'
            if tokens[*i] != Token::LParen {
                panic!("Expected '(' after 'p', got {:?}", tokens[*i]);
            }
            *i += 1; // consume '('
            let value = parse_expr(tokens, i);
            if tokens[*i] != Token::RParen {
                panic!("Expected ')' after print expression, got {:?}", tokens[*i]);
            }
            *i += 1; // consume ')'
            if tokens[*i] != Token::Bang {
                panic!("Expected '!' at end of print statement, got {:?}", tokens[*i]);
            }
            *i += 1;
            Statement::Print(value)
        }
        _ => panic!("Unexpected token: {:?}", tokens[*i])
    }
}

fn parse_expr(tokens: &[Token], i: &mut usize) -> Expr {
    let token = &tokens[*i];
    *i += 1;
    
    match token {
        Token::Number(n) => Expr::Number(*n),
        Token::Bool(b) => Expr::Bool(*b),
        Token::StringLiteral(s) => Expr::StringLit(s.clone()),
        Token::Identifier(name) => Expr::Variable(name.clone()),
        _ => panic!("Expected expression, got {:?}", token)
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<Statement> {
    let mut statements = Vec::new();
    let mut i = 0;
    
    while i < tokens.len() {
        let stmt = parse_statement(&tokens, &mut i);
        statements.push(stmt);
    }
    
    statements
}