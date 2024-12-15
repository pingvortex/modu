use crate::ast::AST;

use std::{collections::HashMap, path::PathBuf};
use crate::utils;

pub fn eval(expr: AST, context: &mut HashMap<String, AST>) -> Result<AST, String> {
    match expr {
        AST::Call { name, args, line: _ } => {
            match name.as_str() {
                "print" => {
                    if args.len() == 1 {
                        match eval(args[0].clone(), context)? {
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

                            AST::PropertyAccess { object, property, line } => {
                                match object {
                                    Some(name) => {
                                        match context.get(&name) {
                                            Some(value) => {
                                                match value {
                                                    AST::Object { properties, line: _ } => {
                                                        match properties.get(property.as_ref().unwrap()) {
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
                                                                return Err(format!("Property {:?} not found", property));
                                                            }
                                                        }
                                                    }
                                    
                                                    _ => {
                                                        return Err(format!("{} is not an object", name));
                                                    }
                                                }
                                            }
                                    
                                            None => {
                                                return Err(format!("Variable {} not found", name));
                                            }
                                        }
                                    }

                                    None => {
                                        return Err("Object not found".to_string());
                                    }
                                }
                            }
    
                            _ => {
                                println!("{:?}", eval(args[0].clone(), context)?);
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
                                            new_context.insert(arg.clone(), eval(args[i].clone(), &mut new_context.clone())?);
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
                        let val = eval(*value, context)?;
                        context.insert(name, val);
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

        AST::Import { file, as_, line } => {
            let args = std::env::args().collect::<Vec<String>>();

            let path: PathBuf;

            if args.len() > 2 {
                path = std::path::Path::new(&args[2]).parent().unwrap().join(file.unwrap().replace("\"", ""));
            } else {
                path = file.unwrap().replace("\"", "").into();
            }

            match std::fs::read_to_string(&path) {
                Ok(file) => {
                    let mut new_context = context.clone();

                    match crate::parser::parse(&file, &mut new_context) {
                        Ok(_) => {
                            let mut properties = HashMap::new();

                            for (name, value) in new_context {
                                properties.insert(name, value);
                            }

                            context.insert(as_.unwrap(), AST::Object { properties, line });
                        }

                        Err(e) => {
                            return Err(e.0);
                        }
                    }
                }

                Err(e) => {
                    dbg!(path);

                    return Err(e.to_string());
                }
            }
            
        }

        AST::PropertyCall { object, property, args, line } => {
            match object {
                Some(name) => {
                    match context.get(&name) {
                        Some(value) => {
                            match value {
                                AST::Object { properties, line } => {
                                    match properties.get(property.as_ref().unwrap()) {
                                        Some(value) => {
                                            match value {
                                                AST::Function { name, args: f_args, body, line } => {
                                                    if args.len() == f_args.len() {
                                                        let mut new_context = context.clone();

                                                        for (i, arg) in f_args.iter().enumerate() {
                                                            new_context.insert(arg.clone(), args[i].clone());
                                                        }

                                                        for expr in body {
                                                            eval(expr.clone(), &mut new_context)?;
                                                        }
                                                    } else {
                                                        return Err(format!("{} takes {} arguments", name, args.len()));
                                                    }
                                                }

                                                _ => {
                                                    return Err(format!("{} of {} is not a function", property.as_ref().unwrap(), name));
                                                }
                                            }
                                        }

                                        None => {
                                            return Err(format!("Property {} not found in object {}", property.as_ref().unwrap(), name));
                                        }
                                    }
                                }

                                _ => {
                                    return Err(format!("{} is not an object", name));
                                }
                            }
                        }

                        None => {
                            return Err(format!("Object {} not found", name));
                        }
                    }
                }

                None => {
                    return Err("Object appears to be null".to_string());
                }
            }
        }

        AST::IsEqual { left, right, line } => {
            match (eval(*left, context)?, eval(*right, context)?) {
                (AST::Number(l), AST::Number(r)) => {
                    return Ok(AST::Boolean(l == r));
                }

                (AST::Float(l), AST::Float(r)) => {
                    return Ok(AST::Boolean(l == r));
                }

                (AST::String(l), AST::String(r)) => {
                    return Ok(AST::Boolean(l == r));
                }

                (AST::Boolean(l), AST::Boolean(r)) => {
                    return Ok(AST::Boolean(l == r));
                }

                _ => {
                    return Ok(AST::Boolean(false));
                }
            }
        }

        AST::IsUnequal { left, right, line } => {
            match (eval(*left, context)?, eval(*right, context)?) {
                (AST::Number(l), AST::Number(r)) => {
                    return Ok(AST::Boolean(l != r));
                }

                (AST::Float(l), AST::Float(r)) => {
                    return Ok(AST::Boolean(l != r));
                }

                (AST::String(l), AST::String(r)) => {
                    return Ok(AST::Boolean(l != r));
                }

                (AST::Boolean(l), AST::Boolean(r)) => {
                    return Ok(AST::Boolean(l != r));
                }

                _ => {
                    return Ok(AST::Boolean(true));
                }
            }
        }

        AST::IfStatement { condition, body, line } => {
            match eval(*condition, context)? {
                AST::Boolean(b) => {
                    if b {
                        for expr in body {
                            eval(expr, context)?;
                        }
                    }
                }

                _ => {
                    return Err("If statement condition must return a boolean".to_string());
                }
            }
        }

        AST::Number(_) | AST::Boolean(_) | AST::Float(_) | AST::Object { .. } | AST::Null => {
            return Ok(expr);
        }

        AST::String(value) => {
            return Ok(AST::String(value.replace("\"", "")));
        }

        AST::Addition { left, right, line } => {
            match (eval(*left.clone(), context)?, eval(*right.clone(), context)?) {
                (AST::Number(l), AST::Number(r)) => {
                    return Ok(AST::Number(l + r));
                }

                (AST::Float(l), AST::Float(r)) => {
                    return Ok(AST::Float(l + r));
                }

                (AST::Number(l), AST::Float(r)) => {
                    return Ok(AST::Float(l as f64 + r));
                }

                (AST::Float(l), AST::Number(r)) => {
                    return Ok(AST::Float(l + r as f64));
                }

                (AST::String(l), AST::String(r)) => {
                    return Ok(AST::String(format!("{}{}", l, r)));
                }

                _ => {
                    return Err(format!("Cannot add {:?} and {:?}", eval(*left, context)?, eval(*right, context)?));
                }
            }
        }

        AST::Subtraction { left, right, line } => {
            match (eval(*left.clone(), context)?, eval(*right.clone(), context)?) {
                (AST::Number(l), AST::Number(r)) => {
                    return Ok(AST::Number(l - r));
                }

                (AST::Float(l), AST::Float(r)) => {
                    return Ok(AST::Float(l - r));
                }

                (AST::Number(l), AST::Float(r)) => {
                    return Ok(AST::Float(l as f64 - r));
                }

                (AST::Float(l), AST::Number(r)) => {
                    return Ok(AST::Float(l - r as f64));
                }

                (AST::Null, AST::Number(r)) => {
                    return Ok(AST::Number(-r));
                }

                (AST::Null, AST::Float(r)) => {
                    return Ok(AST::Float(-r));
                }

                (AST::Number(l), AST::Null) => {
                    return Ok(AST::Number(l));
                }

                (AST::Float(l), AST::Null) => {
                    return Ok(AST::Float(l));
                }

                _ => {
                    return Err(format!("Cannot subtract {:?} and {:?}", eval(*left, context)?, eval(*right, context)?));
                }
            }
        }
        

        AST::Identifer(name) => {
            match context.get(&name) {
                Some(value) => {
                    return Ok(value.clone());
                }

                None => {
                    return Ok(AST::Null);
                }
            }
        }

        _ => {
            return Err(format!("Unknown expression, got {:?}", expr));
        }
    }

    Ok(AST::Null)
}