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

                            AST::Float(f) => {
                                println!("{}", f);
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

                                            AST::Float(f) => {
                                                println!("{}", f);
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
                                        return Err(format!("Variable {} not found", name));
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
                    match context.get(&name) {
                        Some(value) => {
                            match value {
                                AST::Function { name: _, args: f_args, body, line: _ } => {
                                    if args.len() == f_args.len() {
                                        let mut new_context = context.clone();

                                        for (i, arg) in f_args.iter().enumerate() {
                                            new_context.insert(arg.clone(), args[i].clone());
                                        }

                                        for expr in body {
                                            eval(expr.clone(), &mut new_context)?;
                                        }
                                    } else {
                                        return Err(format!("{} takes {} arguments", name, f_args.len()));
                                    }
                                }

                                _ => {
                                    return Err(format!("{} is not a function", name));
                                }
                            }
                        }

                        None => {
                            return Err(format!("Function {} not found", name));
                        }
                    }
                }
            }
        }

        AST::LetDeclaration { name, value, line: _ } => {
            if utils::is_reserved(name.as_ref().unwrap_or(&"".to_string())) {
                return Err(format!("{} is a reserved keyword", name.as_ref().unwrap()));
            }

            if let Some(name) = name {
                
                match *value {
                    AST::Identifer(i_name) => {
                        match context.get(&i_name) {
                            Some(value) => {
                                context.insert(name, value.clone());
                            }

                            None => {
                                return Err(format!("Variable {} not found", i_name));
                            }
                        }
                    }

                    _ => {
                        context.insert(name, *value);
                    }
                }
            }
        }

        AST::Function { name, args, body, line: _ } => {
            context.insert(name.clone(), AST::Function { name, args, body, line: 0 });
        }

        AST::Semicolon => {
            return Ok(AST::Null);
        }

        _ => {
            return Err(format!("Unknown expression, got {:?}", expr));
        }
    }

    Ok(AST::Null)
}