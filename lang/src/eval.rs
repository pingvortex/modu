use crate::ast::AST;

use std::collections::HashMap;
use crate::utils;

pub fn eval(expr: AST, context: &mut HashMap<String, AST>) -> Result<AST, String> {
    match expr {
        AST::Call { name, args, line: _ } => {
            match name.as_str() {
                "print" => {
                    if args.len() == 1 {
                        match args[0].clone() {
                            AST::String(s) => {
                                println!("{}", s.replace("\"", ""));
                            }
    
                            AST::Number(n) => {
                                println!("{}", n);
                            }

                            AST::Boolean(b) => {
                                println!("{}", b);
                            }
    
                            AST::Identifer(name) => {
                                match context.get(&name) {
                                    Some(value) => {
                                        match value {
                                            AST::String(s) => {
                                                println!("{}", s.replace("\"", ""));
                                            }
    
                                            AST::Number(n) => {
                                                println!("{}", n);
                                            }

                                            AST::Boolean(b) => {
                                                println!("{}", b);
                                            }
    
                                            _ => {
                                                println!("{:?}", value);
                                            }
                                        }
                                    }
    
                                    None => {
                                        return Err(format!("Unknown variable: {}", name));
                                    }
                                }
                            }
    
                            _ => {
                                println!("{:?}", args[0]);
                            }
                        }
                    } else {
                        return Err("print() takes one argument".to_string());
                    }
                }

                "exit" => {
                    println!("Exiting...");
                    std::process::exit(0);
                }

                _ => {
                    return Err(format!("Unknown function: {}", name));
                }
            }
        }

        AST::LetDeclaration { name, value, line: _ } => {
            if utils::is_reserved(name.as_ref().unwrap_or(&"".to_string())) {
                return Err(format!("{} is a reserved keyword", name.as_ref().unwrap()));
            }

            if let Some(name) = name {
                context.insert(name, *value);
            }
        }

        _ => {
            return Err("Unknown expression".to_string());
        }
    }

    Ok(AST::Null)
}