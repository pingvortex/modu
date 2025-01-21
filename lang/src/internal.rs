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
    match eval(args[0].clone(), context) {
        Ok(v) => {
            match v {
                AST::String(value) => {
                    match value.parse::<i64>() {
                        Ok(value) => {return Ok(AST::Number(value));},
                        Err(_) => (),
                    }

                    match value.parse::<f64>() {
                        Ok(value) => {return Ok(AST::Number(value as i64));},
                        Err(_) => (),
                    }

                    return Err("int() requires a string or boolean".to_string());
                }

                AST::Boolean(value) => Ok(AST::Number(if value {1} else {0})),
        
                AST::Number(value) => Ok(AST::Number(value)),
        
                _ => Err("int() requires a string or boolean".to_string())
            }
        }

        Err(e) => Err(e)
    }
}

pub fn float(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<AST, String> {
    match eval(args[0].clone(), context) {
        Ok(v) => {
            match v {
                AST::String(value) => {
                    match value.parse::<f64>() {
                        Ok(value) => Ok(AST::Float(value)),
                        Err(_) => Err("float() requires a string or boolean".to_string())
                    }
                }

                AST::Boolean(value) => Ok(AST::Float(if value {1.0} else {0.0})),
        
                AST::Number(value) => Ok(AST::Float(value as f64)),
                AST::Float(value) => Ok(AST::Float(value)),
        
                _ => Err("float() requires a string or boolean".to_string())
            }
        }

        Err(e) => Err(e)
    }
}

pub fn str(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<AST, String> {
    match eval(args[0].clone(), context) {
        Ok(v) => {
            match v {
                AST::String(value) => Ok(AST::String(value)),
        
                AST::Number(value) => Ok(AST::String(value.to_string())),
                AST::Float(value) => Ok(AST::String(value.to_string())),
                AST::Boolean(value) => Ok(AST::String(value.to_string())),
                AST::Null => Ok(AST::String("null".to_string())),
        
                _ => Err("str() requires a string, number or boolean".to_string())
            }
        }

        Err(e) => Err(e)
    }
}

pub fn exit(_: Vec<AST>, _: &mut HashMap<String, AST>) -> Result<AST, String> {
    std::process::exit(0);
}