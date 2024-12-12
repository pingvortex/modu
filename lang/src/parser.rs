use crate::ast::AST;
use crate::lexer::{Token, LexingError};
use crate::eval::eval;

use logos::Logos;
use std::collections::HashMap;

pub fn parse_line(input: &str, context: &mut HashMap<String, AST>) -> Result<AST, String> {
    let verbose = std::env::args().collect::<Vec<String>>()
                            .iter().any(|arg| arg == "--verbose");

    let mut lexer = Token::lexer(input);
    let mut ast = Vec::new();

    while let Some(token) = lexer.next() {
        match token {
            Ok(Token::Let) => {
                ast.push(AST::LetDeclaration {
                    name: None,
                    value: Box::new(AST::Null),
                });
            }

            Ok(Token::Identifer) => {
                let value = ast.pop().unwrap_or(AST::Null);

                if let AST::LetDeclaration { name, value } = value {
                    if name.is_none() {
                        ast.push(AST::LetDeclaration {
                            name: Some(lexer.slice().to_string()),
                            value,
                        });
                    } else {
                        let has_value = context.contains_key(&lexer.slice().to_string());
                        let needs_value = value == Box::new(AST::Null);
                        

                        if needs_value {
                            if has_value {
                                ast.push(AST::LetDeclaration {
                                    name: name,
                                    value: Box::new(context.get(&lexer.slice().to_string()).unwrap().clone()),
                                });
                            } else {
                                return Err(format!("Variable {} not found", lexer.slice()));
                            }
                        } else {
                            return Err(format!("Unexpected identifier: {}", lexer.slice()));
                        }
                    }
                } else {
                    if let AST::Call { name, mut args } = value {
                        args.push(AST::Identifer(lexer.slice().to_string()));
                        ast.push(AST::Call {
                            name,
                            args,
                        });
                    } else {
                        ast.push(AST::Identifer(lexer.slice().to_string()));
                    }
                }
            }

            Ok(Token::Assign) => {
                if let Some(AST::LetDeclaration { name, value }) = ast.pop() {                    
                    ast.push(AST::LetDeclaration {
                        name,
                        value: Box::new(AST::Null),
                    });
                } else {
                    return Err("Expected a let declaration before '='".to_string());
                }
            }

            Ok(Token::LParen) => {
                if let Some(AST::Identifer(name)) = ast.pop() {
                    ast.push(AST::Call {
                        name,
                        args: Vec::new(),
                    });
                } else {
                    return Err("Expected an identifier before '(...)'".to_string());
                }
            }

            Ok(Token::String) => {
                let value = ast.pop().unwrap();

                if let AST::Call { name, mut args } = value {
                    ast.push(AST::Call {
                        name,
                        args: vec![AST::String(lexer.slice().to_string())],
                    });
                } else {
                    if let AST::LetDeclaration { name, mut value } = value {
                        ast.push(AST::LetDeclaration {
                            name,
                            value: Box::new(AST::String(lexer.slice().to_string())),
                        });
                    } else {
                        return Err("Expected a call or let declaration before a string".to_string());
                    }
                }
            }

            Ok(Token::Number) => {
                let value = ast.pop().unwrap();

                if let AST::Call { name, mut args } = value {
                    ast.push(AST::Call {
                        name,
                        args: vec![AST::Number(lexer.slice().parse().unwrap())],
                    });
                } else {
                    if let AST::LetDeclaration { name, mut value } = value {
                        ast.push(AST::LetDeclaration {
                            name,
                            value: Box::new(AST::Number(lexer.slice().parse().unwrap())),
                        });
                    } else {
                        return Err("Expected a call or let declaration before a number".to_string());
                    }
                }
            }

            Ok(Token::Boolean) => {
                let value = ast.pop().unwrap();

                if let AST::Call { name, mut args } = value {
                    ast.push(AST::Call {
                        name,
                        args: vec![AST::Boolean(lexer.slice() == "true")],
                    });
                } else {
                    if let AST::LetDeclaration { name, mut value } = value {
                        ast.push(AST::LetDeclaration {
                            name,
                            value: Box::new(AST::Boolean(lexer.slice() == "true")),
                        });
                    } else {
                        return Err("Expected a call or let declaration before a boolean".to_string());
                    }
                }
            }

            Ok(Token::RParen) => {
                if let Some(AST::Call { name, args }) = ast.pop() {
                    ast.push(AST::Call {
                        name,
                        args,
                    });
                } else {
                    return Err("Expected a call before ')'".to_string());
                }
            }

            Err(_) => {
                match &lexer.extras {
                    LexingError::UnexpectedToken => {
                        return Err(format!("Unexpected token: {:?}", lexer.slice()));

                    }

                    LexingError::ExpectedToken => {
                        return Err(format!("Expected token: {:?}", lexer.slice()));
                    }
                }
            }

            _ => {}
        }
    }

    if verbose {
        println!("{:?}", ast);
    }

    if ast.len() == 1 {
        match ast[0].clone() {
            AST::LetDeclaration { name, value } => {
                let result = eval(AST::LetDeclaration { name, value }, context);
                if result.is_err() {
                    return result;
                }
            }

            AST::Call { name, args } => {
                let result = eval(AST::Call { name, args }, context);

                if result.is_err() {
                    return result;
                }
            }

            _ => {}
        }

        Ok(ast.pop().unwrap())
    } else {
        Ok(AST::Null)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn let_str() {
        let mut context = HashMap::new();
        let result = parse_line("let x = \"test\"", &mut context);

        assert_eq!(result, Ok(AST::LetDeclaration {
            name: Some("x".to_string()),
            value: Box::new(AST::String("\"test\"".to_string())),
        }));
    }


    #[test]
    fn let_number() {
        let mut context = HashMap::new();
        let result = parse_line("let x = 10", &mut context);

        assert_eq!(result, Ok(AST::LetDeclaration {
            name: Some("x".to_string()),
            value: Box::new(AST::Number(10)),
        }));
    }

    #[test]
    fn print_str() {
        let mut context = HashMap::new();
        let result = parse_line("print(\"Hello, world!\")", &mut context);

        assert_eq!(result, Ok(AST::Call {
            name: "print".to_string(),
            args: vec![AST::String("\"Hello, world!\"".to_string())],
        }));
    }

    #[test]
    fn print_number() {
        let mut context = HashMap::new();
        let result = parse_line("print(10)", &mut context);

        assert_eq!(result, Ok(AST::Call {
            name: "print".to_string(),
            args: vec![AST::Number(10)],
        }));
    }

    #[test]
    fn print_var() {
        let mut context = HashMap::new();
        parse_line("let x = 10", &mut context);

        let result = parse_line("print(x)", &mut context);

        assert_eq!(result, Ok(AST::Call {
            name: "print".to_string(),
            args: vec![AST::Identifer("x".to_string())],
        }));
    }

    #[test]
    fn print_unknown_var() {
        let mut context = HashMap::new();
        let result = parse_line("print(x)", &mut context);

        assert_eq!(result, Err("Unknown variable: x".to_string()));
    }
}