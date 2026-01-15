mod lexer;
mod parser;

use lexer::lex;
use parser::parse;

fn main() {
    let code = "v x = 5!";
    let tokens = lex(code);
    println!("Tokens: {:?}", tokens);
    
    let ast = parse(tokens);
    println!("AST: {:?}", ast);
}