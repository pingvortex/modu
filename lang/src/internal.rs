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

pub fn input(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<AST, String> {
    if args.len() > 0 {
        use std::io::Write;

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

        std::io::stdout().flush().unwrap();
    }

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    Ok(AST::String(input.trim().to_string()))
}

pub fn int(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<AST, String> {
    if args.len() != 1 {
        return Err("int() requires exactly one argument".to_string());
    }

    match eval(args[0].clone(), context) {
        Ok(v) => {
            match v {
                AST::String(value) => {
                    match value.parse::<i64>() {
                        Ok(value) => Ok(AST::Number(value)),
                        Err(_) => Err("Could not parse string to int".to_string())
                    }
                }
        
                AST::Number(value) => Ok(AST::Number(value)),
        
                _ => Err("int() requires a string or number".to_string())
            }
        }

        Err(e) => Err(e)
    }
}

pub fn exit(_: Vec<AST>, _: &mut HashMap<String, AST>) -> Result<AST, String> {
    std::process::exit(0);
}