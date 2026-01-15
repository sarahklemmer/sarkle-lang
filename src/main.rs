mod lexer;
mod parser;
use lexer::{Token, lex};

fn main() {
    let code = r#"v name = "hello"!"#;
    let tokens = lex(code);
    println!("{:?}", tokens);
}
