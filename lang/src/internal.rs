// internal modu functions

use std::collections::HashMap;

use crate::ast::AST;
use crate::eval::eval;

pub fn print(ast: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<AST, String> {
    match eval(ast[0].clone(), context) {
        Ok(ast) => {
            println!("{}", ast);
            return Ok(ast);
        }

        Err(e) => {
            return Err(e);
        }
    }
}

pub fn exit(_: Vec<AST>, _: &mut HashMap<String, AST>) -> Result<AST, String> {
    std::process::exit(0);
}