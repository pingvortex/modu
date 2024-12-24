// internal modu functions

use std::collections::HashMap;

use crate::ast::AST;
use crate::eval::eval;

pub fn print(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<AST, String> {
    for arg in args {
        match eval(arg, context) {
            Ok(value) => {
                print!("{}", value);
            }

            Err(e) => {
                return Err(e);
            }
        }
    }

    println!();

    Ok(AST::Null)
}

pub fn exit(_: Vec<AST>, _: &mut HashMap<String, AST>) -> Result<AST, String> {
    std::process::exit(0);
}