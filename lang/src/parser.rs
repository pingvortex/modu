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
    let mut function = false;

    for line in input.split("\n") {
        current_line += 1;

        let mut lexer = Token::lexer(line);

        if verbose {
            dbg!(lexer.clone().spanned().collect::<Vec<_>>());
        }

        let mut temp_ast = Vec::new();
        let mut function_starts = false;

        while let Some(token) = lexer.next() {
            match token {
                Ok(Token::Let) => {
                    temp_ast.push(AST::LetDeclaration {
                        name: None,
                        value: Box::new(AST::Null),
                        line: current_line,
                    });
                }

                Ok(Token::Fn) => {
                    temp_ast.push(AST::Function {
                        name: String::new(),
                        args: Vec::new(),
                        body: Vec::new(),
                        line: current_line,
                    });
                }
    
                Ok(Token::Identifer) => {
                    let value = temp_ast.pop().unwrap_or(AST::Null);
                    match value {
                        AST::LetDeclaration { name, value, line } => {
                            if name.is_none() {
                                temp_ast.push(AST::LetDeclaration {
                                    name: Some(lexer.slice().to_string()),
                                    value,
                                    line,
                                });
                            } else {
                                temp_ast.push(AST::LetDeclaration {
                                    name,
                                    value: Box::new(AST::Identifer(lexer.slice().to_string())),
                                    line,
                                });
                            }
                        }

                        AST::Call { name, mut args, line } => {
                            args.push(AST::Identifer(lexer.slice().to_string()));

                            temp_ast.push(AST::Call {
                                name,
                                args,
                                line,
                            });
                        }

                        AST::Function { name, mut args, body, line } => {
                            if name.is_empty() {
                                temp_ast.push(AST::Function {
                                    name: lexer.slice().to_string(),
                                    args,
                                    body,
                                    line,
                                });
                            } else {
                                args.push(lexer.slice().to_string());

                                temp_ast.push(AST::Function {
                                    name,
                                    args,
                                    body,
                                    line,
                                });
                            }
                        }

                        _ => {
                            temp_ast.push(AST::Identifer(lexer.slice().to_string()));
                        }
                    }
                }
    
                Ok(Token::Assign) => {
                    if let Some(AST::LetDeclaration { name, value, line }) = temp_ast.pop() {                    
                        temp_ast.push(AST::LetDeclaration {
                            name,
                            value: Box::new(AST::Null),
                            line,
                        });
                    } else {
                        return Err(("Expected a let declaration before '='".to_string(), current_line));
                    }
                }
    
                Ok(Token::LParen) => {
                    match temp_ast.pop().unwrap_or(AST::Null) {
                        AST::Identifer(name) => {
                            temp_ast.push(AST::Call {
                                name,
                                args: Vec::new(),
                                line: current_line,
                            });
                        }

                        AST::Function { name, args, body, line } => {
                            temp_ast.push(AST::Function {
                                name,
                                args,
                                body,
                                line,
                            });
                        }

                        _ => {
                            return Err(("Expected an identifier before '()'".to_string(), current_line));
                        }
                    }
                }
    
                Ok(Token::String) => {
                    let value = temp_ast.pop().unwrap_or(AST::Null);
    
                    match value {
                        AST::Call { name, mut args, line } => {
                            args.push(AST::String(lexer.slice().to_string()));

                            temp_ast.push(AST::Call {
                                name,
                                args,
                                line,
                            });
                        }

                        AST::LetDeclaration { name, value, line } => {
                            temp_ast.push(AST::LetDeclaration {
                                name,
                                value: Box::new(AST::String(lexer.slice().to_string())),
                                line,
                            });
                        }

                        _ => {
                            return Err(("Expected a call or let declaration before a string".to_string(), current_line));
                        }
                    }
                }
    
                Ok(Token::Number) => {
                    let value = temp_ast.pop().unwrap_or(AST::Null);

                    match value {
                        AST::Call { name, mut args, line } => {
                            args.push(AST::Number(lexer.slice().parse().unwrap()));

                            temp_ast.push(AST::Call {
                                name,
                                args,
                                line,
                            });
                        }

                        AST::LetDeclaration { name, value, line } => {
                            temp_ast.push(AST::LetDeclaration {
                                name,
                                value: Box::new(AST::Number(lexer.slice().parse().unwrap())),
                                line,
                            });
                        }

                        _ => {
                            return Err(("Expected a call or let declaration before a number".to_string(), current_line));
                        }
                    }
                }
    
                Ok(Token::Boolean) => {
                    let value = temp_ast.pop().unwrap_or(AST::Null);

                    match value {
                        AST::Call { name, mut args, line } => {
                            args.push(AST::Boolean(lexer.slice() == "true"));

                            temp_ast.push(AST::Call {
                                name,
                                args,
                                line,
                            });
                        }

                        AST::LetDeclaration { name, value, line } => {
                            temp_ast.push(AST::LetDeclaration {
                                name,
                                value: Box::new(AST::Boolean(lexer.slice() == "true")),
                                line,
                            });
                        }

                        _ => {
                            return Err(("Expected a call or let declaration before a boolean".to_string(), current_line));
                        }
                    }
                }
    
                Ok(Token::Float) => {
                    let value = temp_ast.pop().unwrap_or(AST::Null);
    
                    match value {
                        AST::Call { name, mut args, line } => {
                            args.push(AST::Float(lexer.slice().parse().unwrap()));
    
                            temp_ast.push(AST::Call {
                                name,
                                args,
                                line,
                            });
                        }

                        AST::LetDeclaration { name, value, line } => {
                            temp_ast.push(AST::LetDeclaration {
                                name,
                                value: Box::new(AST::Float(lexer.slice().parse().unwrap())),
                                line,
                            });
                        }

                        _ => {
                            return Err(("Expected a call or let declaration before a float".to_string(), current_line));
                        }
                    }
                }
    
                Ok(Token::RParen) => {
                    match temp_ast.pop().unwrap_or(AST::Null) {
                        AST::Call { name, args, line } => {
                            temp_ast.push(AST::Call {
                                name,
                                args,
                                line,
                            });
                        }

                        AST::Function { name, args, body, line } => {
                            temp_ast.push(AST::Function {
                                name,
                                args,
                                body,
                                line,
                            });
                        }

                        _ => {
                            return Err(("Expected a call or function before ')'".to_string(), current_line));
                        }
                    }
                }
    
                Ok(Token::Semicolon) => {
                    temp_ast.push(AST::Semicolon);
                }

                Ok(Token::LBracket) => {
                    match temp_ast.pop().unwrap_or(AST::Null) {
                        AST::Function { name, args, body, line } => {
                            temp_ast.push(AST::Function {
                                name,
                                args,
                                body,
                                line,
                            });

                            function = true;
                            function_starts = true;
                        }

                        _ => {
                            return Err(("Expected a function before '{'".to_string(), current_line));
                        }
                    }
                }

                Ok(Token::RBracket) => {
                    match temp_ast.pop().unwrap_or(AST::Null) {
                        AST::Function { name, args, body, line } => {
                            temp_ast.push(AST::Function {
                                name,
                                args,
                                body,
                                line,
                            });

                            function = false;
                        }

                        _ => {
                            if function {
                                function = false;
                            } else {
                                return Err(("Expected a function before '}'".to_string(), current_line));
                            }
                        }
                    }
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

        if function && !function_starts {
            let function = ast.pop().unwrap_or(AST::Null);

            match function {
                AST::Function { name, args, mut body, line } => {
                    for expr in temp_ast {
                        body.push(expr);
                    }

                    ast.push(AST::Function {
                        name,
                        args,
                        body,
                        line,
                    });
                }

                _ => {
                    return Err(("Expected a function".to_string(), current_line));
                }
            }
        } else {
            ast.append(&mut temp_ast);
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

            AST::Function { name, args, body, line } => {
                let result = eval(AST::Function { name, args, body, line }, context);

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
    fn let_float() {
        let mut context = HashMap::new();
        let result = parse("let x = 1.123", &mut context);

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn let_unknown_var() {
        let mut context = HashMap::new();
        let result = parse("let x = y", &mut context);

        assert_eq!(result, Err(("Variable y not found".to_string(), 1)));
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
    fn print_float() {
        let mut context = HashMap::new();
        let result = parse("print(1.123)", &mut context);

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
        parse("let x = 10", &mut context).unwrap();

        let result = parse("print(x)", &mut context);

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn print_unknown_var() {
        let mut context = HashMap::new();
        let result = parse("print(x)", &mut context);

        assert_eq!(result, Err(("Variable x not found".to_string(), 1)));
    }
}