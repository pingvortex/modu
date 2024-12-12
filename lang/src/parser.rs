use crate::ast::AST;
use crate::lexer::{Token, LexingError};
use crate::eval::eval;

use logos::Logos;
use std::collections::HashMap;

pub fn parse(input: &str, context: &mut HashMap<String, AST>) -> Result<(), (String, usize)> {
    let verbose = std::env::args().collect::<Vec<String>>()
                            .iter().any(|arg| arg == "--verbose");

    let mut ast = Vec::new();
    let mut line_map = HashMap::new();
    let mut current_line = 0;

    for line in input.split("\n") {
        current_line += 1;

        let mut lexer = Token::lexer(line);

        if verbose {
            dbg!(lexer.clone().spanned().collect::<Vec<_>>());
        }

        while let Some(token) = lexer.next() {
            match token {
                Ok(Token::Let) => {
                    ast.push(AST::LetDeclaration {
                        name: None,
                        value: Box::new(AST::Null),
                        line: current_line,
                    });
                }
    
                Ok(Token::Identifer) => {
                    let value = ast.pop().unwrap_or(AST::Null);
    
                    if let AST::LetDeclaration { name, value, line } = value {
                        if name.is_none() {
                            ast.push(AST::LetDeclaration {
                                name: Some(lexer.slice().to_string()),
                                value,
                                line,
                            });
                        } else {
                            let has_value = context.contains_key(&lexer.slice().to_string());
                            let needs_value = value == Box::new(AST::Null);
                            
    
                            if needs_value {
                                if has_value {
                                    ast.push(AST::LetDeclaration {
                                        name: name,
                                        value: Box::new(context.get(&lexer.slice().to_string()).unwrap().clone()),
                                        line,
                                    });
                                } else {
                                    return Err((format!("Variable {} not found", lexer.slice()), current_line));
                                }
                            } else {
                                return Err((format!("Unexpected identifier: {}", lexer.slice()), current_line));
                            }
                        }
                    } else {
                        if let AST::Call { name, mut args, line } = value {
                            args.push(AST::Identifer(lexer.slice().to_string()));

                            ast.push(AST::Call {
                                name,
                                args,
                                line,
                            });
                        } else {
                            ast.push(AST::Identifer(lexer.slice().to_string()));
                        }
                    }
                }
    
                Ok(Token::Assign) => {
                    if let Some(AST::LetDeclaration { name, value, line }) = ast.pop() {                    
                        ast.push(AST::LetDeclaration {
                            name,
                            value: Box::new(AST::Null),
                            line,
                        });
                    } else {
                        return Err(("Expected a let declaration before '='".to_string(), current_line));
                    }
                }
    
                Ok(Token::LParen) => {
                    if let Some(AST::Identifer(name)) = ast.pop() {
                        ast.push(AST::Call {
                            name,
                            args: Vec::new(),
                            line: current_line,
                        });
                    } else {
                        return Err(("Expected an identifier before '(...)'".to_string(), current_line));
                    }
                }
    
                Ok(Token::String) => {
                    let value = ast.pop().unwrap_or(AST::Null);
    
                    if let AST::Call { name, args, line } = value {
                        ast.push(AST::Call {
                            name,
                            args: vec![AST::String(lexer.slice().to_string())],
                            line,
                        });
                    } else {
                        if let AST::LetDeclaration { name, value, line } = value {
                            ast.push(AST::LetDeclaration {
                                name,
                                value: Box::new(AST::String(lexer.slice().to_string())),
                                line
                            });
                        } else {
                            return Err(("Expected a call or let declaration before a string".to_string(), current_line));
                        }
                    }
                }
    
                Ok(Token::Number) => {
                    let value = ast.pop().unwrap_or(AST::Null);
    
                    if let AST::Call { name, args, line } = value {
                        ast.push(AST::Call {
                            name,
                            args: vec![AST::Number(lexer.slice().parse().unwrap())],
                            line
                        });
                    } else {
                        if let AST::LetDeclaration { name, value, line } = value {
                            ast.push(AST::LetDeclaration {
                                name,
                                value: Box::new(AST::Number(lexer.slice().parse().unwrap())),
                                line,
                            });
                        } else {
                            return Err(("Expected a call or let declaration before a number".to_string(), current_line));
                        }
                    }
                }
    
                Ok(Token::Boolean) => {
                    let value = ast.pop().unwrap_or(AST::Null);
    
                    if let AST::Call { name, args, line } = value {
                        ast.push(AST::Call {
                            name,
                            args: vec![AST::Boolean(lexer.slice() == "true")],
                            line
                        });
                    } else {
                        if let AST::LetDeclaration { name, value, line } = value {
                            ast.push(AST::LetDeclaration {
                                name,
                                value: Box::new(AST::Boolean(lexer.slice() == "true")),
                                line
                            });
                        } else {
                            return Err(("Expected a call or let declaration before a boolean".to_string(), current_line));
                        }
                    }
                }
    
                Ok(Token::Float) => {
                    let value = ast.pop().unwrap_or(AST::Null);
    
                    if let AST::Call { name, args, line } = value {
                        ast.push(AST::Call {
                            name,
                            args: vec![AST::Float(lexer.slice().parse().unwrap())],
                            line
                        });
                    } else {
                        if let AST::LetDeclaration { name, value, line } = value {
                            ast.push(AST::LetDeclaration {
                                name,
                                value: Box::new(AST::Float(lexer.slice().parse().unwrap())),
                                line,
                            });
                        } else {
                            return Err(("Expected a call or let declaration before a float".to_string(), current_line));
                        }
                    }
                }
    
                Ok(Token::RParen) => {
                    if let Some(AST::Call { name, args, line }) = ast.pop() {
                        ast.push(AST::Call {
                            name,
                            args,
                            line,
                        });
                    } else {
                        return Err(("Expected a call before ')'".to_string(), current_line));
                    }
                }
    
                Ok(Token::Semicolon) => {
                    ast.push(AST::Semiclon);
                }
    
                Err(_) => {
                    match &lexer.extras {
                        LexingError::UnexpectedToken => {
                            return Err((format!("Unexpected token: {:?}", lexer.slice()), current_line));
    
                        }
    
                        LexingError::ExpectedToken => {
                            return Err((format!("Expected token: {:?}", lexer.slice()), current_line));
                        }
                    }
                }
    
                _ => {}
            }

            line_map.insert(current_line, ast.clone());
        }
    
        if verbose {
            println!("{:?}", ast);
        }
    }

    for item in ast.clone() {
        match item {
            AST::LetDeclaration { name, value, line } => {
                let result = eval(AST::LetDeclaration { name, value, line }, context);

                if result.is_err() {
                    return Err((result.err().unwrap(), line));
                }
            }

            AST::Call { name, args, line } => {
                let result = eval(AST::Call { name, args, line }, context);

                if result.is_err() {
                    return Err((result.err().unwrap(), line));
                }
            }
            _ => {}
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn let_str() {
        let mut context = HashMap::new();
        let result = parse("let x = \"test\"", &mut context);

        assert_eq!(result, Ok(()));
    }


    #[test]
    fn let_number() {
        let mut context = HashMap::new();
        let result = parse("let x = 10", &mut context);

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn let_boolean() {
        let mut context = HashMap::new();
        let result = parse("let x = true", &mut context);

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn print_str() {
        let mut context = HashMap::new();
        let result = parse("print(\"Hello, world!\")", &mut context);

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn print_number() {
        let mut context = HashMap::new();
        let result = parse("print(10)", &mut context);

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn print_boolean() {
        let mut context = HashMap::new();
        let result = parse("print(true)", &mut context);

        assert_eq!(result, Ok(()));

    }

    #[test]
    fn print_var() {
        let mut context = HashMap::new();
        parse("let x = 10", &mut context);

        let result = parse("print(x)", &mut context);

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn print_unknown_var() {
        let mut context = HashMap::new();
        let result = parse("print(x)", &mut context);

        assert_eq!(result, Err(("Unknown variable: x".to_string(), 1)));
    }
}