use crate::parser::{Statement, Expr};
use std::collections::HashMap;

pub fn interpret(statements: Vec<Statement>) {
    let mut variables: HashMap<String, ???> = HashMap::new();
    
    for stmt in statements {
        execute_statement(stmt, &mut variables);
    }
}

fn execute_statement(stmt: Statement, variables: &mut HashMap<String, ???>) {
    match stmt {
        Statement::VarDecl { name, value } => {
            // evaluate the expression and store it in variables
        }
        Statement::Print(expr) => {
            // evaluate the expression and print it
        }
    }
}

fn eval_expr(expr: Expr, variables: &HashMap<String, ???>) -> ??? {
    // evaluate an expression and return its value
}