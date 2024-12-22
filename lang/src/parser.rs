use crate::ast::AST;
use crate::lexer::{Token, LexingError};
use crate::eval::eval;

use logos::Logos;
use std::collections::HashMap;

pub fn handle_nested_ast(mut ast: Vec<AST>, temp_ast: Vec<AST>, current_line: usize) -> Result<Vec<AST>, (String, usize)> {
    if ast.is_empty() {
        return Ok(temp_ast);
    }

    let last = ast.pop().unwrap();

    match last {
        AST::Function { name, args, mut body, line } => {
            if let Some(last_body_expr) = body.pop() {
                match last_body_expr {
                    AST::IfStatement { condition, body: if_body, line: if_line } => {
                        let updated_body = handle_nested_ast(if_body, temp_ast, current_line)?;

                        body.push(AST::IfStatement {
                            condition,
                            body: updated_body,
                            line: if_line,
                        });
                    }

                    AST::Null => {
                        body.extend(temp_ast);
                    }

                    other => {
                        body.push(other);
                        body.extend(temp_ast);
                    }
                }
            } else {
                body.extend(temp_ast);
            }

            ast.push(AST::Function {
                name,
                args,
                body,
                line,
            });

            Ok(ast)
        }

        AST::IfStatement { condition, mut body, line } => {
            if let Some(last_body_expr) = body.pop() {
                match last_body_expr {
                    AST::IfStatement { condition: if_condition, body: if_body, line: if_line } => {
                        let updated_body = handle_nested_ast(if_body, temp_ast, current_line)?;

                        body.push(AST::IfStatement {
                            condition: if_condition,
                            body: updated_body,
                            line: if_line,
                        });
                    }

                    AST::Null => {
                        body.extend(temp_ast);
                    }

                    other => {
                        body.push(other);
                        body.extend(temp_ast);
                    }
                }
            } else {
                body.extend(temp_ast);
            }

            ast.push(AST::IfStatement {
                condition,
                body,
                line,
            });

            Ok(ast)
        }

        _ => {
            ast.push(last);
            ast.extend(temp_ast);

            Ok(ast)
        }
    }
}

pub fn parse(input: &str, context: &mut HashMap<String, AST>) -> Result<(), (String, usize)> {
    let verbose = std::env::args().collect::<Vec<String>>()
                            .iter().any(|arg| arg == "--verbose");

    let mut ast = Vec::new();
    let mut line_map = HashMap::new();
    let mut current_line = 0;
    let mut bodies_deep = 0;

    for line in input.split("\n") {
        current_line += 1;

        let mut lexer = Token::lexer(line);

        if verbose {
            dbg!(lexer.clone().spanned().collect::<Vec<_>>());
        }

        let mut temp_ast = Vec::new();
        let mut body_starts = false;

        while let Some(token) = lexer.next() {
            match token {
                Ok(Token::Import) => {
                    temp_ast.push(AST::Import {
                        file: None,
                        as_: None,
                        line: current_line,
                    });
                }

                Ok(Token::As) => {
                    let value = temp_ast.pop().unwrap_or(AST::Null);

                    match value {
                        AST::Import { file, line, .. } => {
                            if file.is_none() {
                                return Err(("Expected a file before 'as'".to_string(), current_line));
                            }

                            temp_ast.push(AST::Import {
                                file,
                                as_: Some(lexer.slice().to_string()),
                                line,
                            });
                        }

                        _ => {
                            return Err(("Expected an import before 'as'".to_string(), current_line));
                        }
                    }
                }

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

                Ok(Token::If) => {
                    temp_ast.push(AST::IfStatement {
                        condition: Box::new(AST::Null),
                        body: Vec::new(),
                        line: current_line,
                    });
                }

                Ok(Token::IsEqual) => {
                    let value = temp_ast.pop().unwrap_or(AST::Null);

                    match value {
                        AST::Null => {
                            return Err(("Expected an value before '=='".to_string(), current_line));
                        }

                        _ => {
                            match temp_ast.pop().unwrap_or(AST::Null) {
                                AST::Null => {
                                    return Err(("Expected an value before '=='".to_string(), current_line));
                                }

                                AST::IfStatement { mut condition, body, line } => {
                                    condition = Box::new(AST::IsEqual {
                                        left: Box::new(value),
                                        right: Box::new(AST::Null),
                                        line,
                                    });

                                    temp_ast.push(AST::IfStatement {
                                        condition,
                                        body,
                                        line,
                                    });
                                }

                                _ => {
                                    return Err(("Expected an if statement before '=='".to_string(), current_line));
                                }
                            }
                        }
                    }
                }

                Ok(Token::IsUnequal) => {
                    let value = temp_ast.pop().unwrap_or(AST::Null);

                    match value {
                        AST::Null => {
                            return Err(("Expected an value before '!='".to_string(), current_line));
                        }

                        _ => {
                            match temp_ast.pop().unwrap_or(AST::Null) {
                                AST::Null => {
                                    return Err(("Expected an value before '!='".to_string(), current_line));
                                }

                                AST::IfStatement { mut condition, body, line } => {
                                    condition = Box::new(AST::IsUnequal {
                                        left: Box::new(value),
                                        right: Box::new(AST::Null),
                                        line,
                                    });

                                    temp_ast.push(AST::IfStatement {
                                        condition,
                                        body,
                                        line,
                                    });
                                }

                                _ => {
                                    return Err(("Expected an if statement before '!='".to_string(), current_line));
                                }
                            }
                        }
                    }
                }

                Ok(Token::Dot) => {
                    let value = temp_ast.pop().unwrap_or(AST::Null);

                    match value {
                        AST::Identifer(name) => {
                            temp_ast.push(AST::PropertyAccess {
                                object: Some(name),
                                property: None,
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

                        AST::Call { name, mut args, line } => {
                            if args.is_empty() {
                                return Err(("Expected a property before '.' in function call".to_string(), current_line));
                            } else {
                                let arg = args.pop().unwrap();

                                match arg {
                                    AST::Identifer(name) => {
                                        args.push(AST::PropertyAccess {
                                            object: Some(name),
                                            property: None,
                                            line,
                                        });
                                    }

                                    _ => {
                                        return Err(("Expected an identifier before '.'".to_string(), current_line));
                                    }
                                }

                                temp_ast.push(AST::Call {
                                    name,
                                    args,
                                    line,
                                });
                            }
                        }

                        _ => {
                            return Err(("Expected an identifier before '.'".to_string(), current_line));
                        }
                    }
                }
    
                Ok(Token::Identifer) => {
                    let value = temp_ast.pop().unwrap_or(AST::Null);
                    match value {
                        AST::Import { file, as_, line } => {
                            temp_ast.push(AST::Import {
                                file,
                                as_: Some(lexer.slice().to_string()),
                                line,
                            });
                        }

                        AST::LetDeclaration { name, value, line } => {
                            if name.is_none() {
                                temp_ast.push(AST::LetDeclaration {
                                    name: Some(lexer.slice().to_string()),
                                    value,
                                    line,
                                });
                            } else {
                                if AST::Null == *value {
                                    temp_ast.push(AST::LetDeclaration {
                                        name,
                                        value: Box::new(AST::Identifer(lexer.slice().to_string())),
                                        line,
                                    });
                                } else {
                                    if let AST::Addition { left, right, line } = *value {
                                        temp_ast.push(AST::LetDeclaration {
                                            name,
                                            value: Box::new(AST::Addition {
                                                left,
                                                right: Box::new(AST::Identifer(lexer.slice().to_string())),
                                                line,
                                            }),
                                            line,
                                        });
                                    } else {
                                        if let AST::Subtraction { left, right, line } = *value {
                                            temp_ast.push(AST::LetDeclaration {
                                                name,
                                                value: Box::new(AST::Subtraction {
                                                    left,
                                                    right: Box::new(AST::Identifer(lexer.slice().to_string())),
                                                    line,
                                                }),
                                                line,
                                            });
                                        } else {
                                            return Err((format!("Unexpected identifier '{}'", lexer.slice()), current_line));
                                        }
                                    }
                                }
                            }
                        }

                        AST::Call { name, mut args, line } => {
                            match args.pop().unwrap_or(AST::Null) {
                                AST::PropertyAccess { object, property, line } => {
                                    args.push(AST::PropertyAccess {
                                        object,
                                        property: Some(lexer.slice().to_string()),
                                        line,
                                    });

                                    temp_ast.push(AST::Call {
                                        name,
                                        args,
                                        line,
                                    });
                                }

                                AST::Addition { left, right, line } => {
                                    args.push(AST::Addition {
                                        left,
                                        right: Box::new(AST::Identifer(lexer.slice().to_string())),
                                        line,
                                    });

                                    temp_ast.push(AST::Call {
                                        name,
                                        args,
                                        line,
                                    });
                                }

                                AST::Subtraction { left, right, line } => {
                                    args.push(AST::Subtraction {
                                        left,
                                        right: Box::new(AST::Identifer(lexer.slice().to_string())),
                                        line,
                                    });

                                    temp_ast.push(AST::Call {
                                        name,
                                        args,
                                        line,
                                    });
                                }

                                _ => {
                                    args.push(AST::Identifer(lexer.slice().to_string()));

                                    temp_ast.push(AST::Call {
                                        name,
                                        args,
                                        line,
                                    });
                                    
                                }
                            }
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

                        AST::PropertyAccess { object, property, line } => {
                            temp_ast.push(AST::PropertyAccess {
                                object,
                                property: Some(lexer.slice().to_string()),
                                line,
                            });
                        }

                        AST::IfStatement { condition, body, line } => {
                            if let AST::IsEqual { left, right, line } = condition.as_ref() {
                                if let AST::Null = **right {
                                    temp_ast.push(AST::IfStatement {
                                        condition: Box::new(AST::IsEqual {
                                            left: left.clone(),
                                            right: Box::new(AST::Identifer(lexer.slice().to_string())),
                                            line: *line,
                                        }),
                                        body,
                                        line: *line,
                                    });
                                } else {
                                    return Err(("Expected a value before '=='".to_string(), current_line));
                                }
                            } else {
                                if let AST::IsUnequal { left, right, line } = condition.as_ref() {
                                    if let AST::Null = **right {
                                        temp_ast.push(AST::IfStatement {
                                            condition: Box::new(AST::IsUnequal {
                                                left: left.clone(),
                                                right: Box::new(AST::Identifer(lexer.slice().to_string())),
                                                line: *line,
                                            }),
                                            body,
                                            line: *line,
                                        });
                                    } else {
                                        return Err(("Expected a value before '!='".to_string(), current_line));
                                    }
                                } else {
                                    temp_ast.push(AST::IfStatement {
                                        condition,
                                        body,
                                        line,
                                    });
        
                                    temp_ast.push(AST::Identifer(lexer.slice().to_string()));
                                }
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

                Ok(Token::Comma) => {
                    let value = temp_ast.pop().unwrap_or(AST::Null);

                    match value {
                        AST::Call { name, mut args, line } => {
                            args.push(AST::Comma);

                            temp_ast.push(AST::Call {
                                name,
                                args,
                                line,
                            });
                        }

                        AST::PropertyCall { object, property, mut args, line } => {
                            args.push(AST::Comma);

                            temp_ast.push(AST::PropertyCall {
                                object,
                                property,
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
                            return Err(("Expected a call or property call before ','".to_string(), current_line));
                        }
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

                        AST::PropertyAccess { object, property, line } => {
                            temp_ast.push(AST::PropertyCall {
                                object,
                                property,
                                args: Vec::new(),
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
                        AST::Import { file, as_, line } => {
                            if file.is_none() {
                                temp_ast.push(AST::Import {
                                    file: Some(lexer.slice().to_string()),
                                    as_,
                                    line,
                                });
                            } else {
                                return Err(("Expected a import state before the file path".to_string(), current_line));
                            }
                        }

                        AST::Call { name, mut args, line } => {
                            let arg = args.pop().unwrap_or(AST::Null);

                            match arg {
                                AST::Addition { left, right, line } => {
                                    args.push(AST::Addition {
                                        left,
                                        right: Box::new(AST::String(lexer.slice().to_string())),
                                        line,
                                    });
                                }

                                AST::Subtraction { left, right, line } => {
                                    args.push(AST::Subtraction {
                                        left,
                                        right: Box::new(AST::String(lexer.slice().to_string())),
                                        line,
                                    });
                                }

                                _ => {
                                    args.push(AST::String(lexer.slice().to_string()));
                                }
                            }

                            temp_ast.push(AST::Call {
                                name,
                                args,
                                line,
                            });
                        }

                        AST::PropertyCall { object, property, mut args, line } => {
                            args.push(AST::String(lexer.slice().to_string()));

                            temp_ast.push(AST::PropertyCall {
                                object,
                                property,
                                args,
                                line,
                            });
                        }

                        AST::LetDeclaration { name, value, line } => {
                            if let AST::Addition { left, right, line } = *value {
                                temp_ast.push(AST::LetDeclaration {
                                    name,
                                    value: Box::new(AST::Addition {
                                        left,
                                        right: Box::new(AST::String(lexer.slice().to_string())),
                                        line,
                                    }),
                                    line,
                                });
                            } else {
                                temp_ast.push(AST::LetDeclaration {
                                    name,
                                    value: Box::new(AST::String(lexer.slice().to_string())),
                                    line,
                                });
                            }
                        }

                        AST::IfStatement { condition, body, line } => {
                            if let AST::IsEqual { left, right, line } = condition.as_ref() {
                                if let AST::Null = **right {
                                    temp_ast.push(AST::IfStatement {
                                        condition: Box::new(AST::IsEqual {
                                            left: left.clone(),
                                            right: Box::new(AST::String(lexer.slice().to_string())),
                                            line: *line,
                                        }),
                                        body,
                                        line: *line,
                                    });
                                } else {
                                    return Err(("Expected a value before '=='".to_string(), current_line));
                                }
                            } else {
                                if let AST::IsUnequal { left, right, line } = condition.as_ref() {
                                    if let AST::Null = **right {
                                        temp_ast.push(AST::IfStatement {
                                            condition: Box::new(AST::IsUnequal {
                                                left: left.clone(),
                                                right: Box::new(AST::String(lexer.slice().to_string())),
                                                line: *line,
                                            }),
                                            body,
                                            line: *line,
                                        });
                                    } else {
                                        return Err(("Expected a value before '!='".to_string(), current_line));
                                    }
                                } else {
                                    temp_ast.push(AST::IfStatement {
                                        condition,
                                        body,
                                        line,
                                    });
        
                                    temp_ast.push(AST::String(lexer.slice().to_string()));
                                }
                            }
                        }

                        _ => {
                            return Err(("Expected a call or let declaration before a string".to_string(), current_line));
                        }
                    }
                }

                Ok(Token::Plus) => {
                    let value = temp_ast.pop().unwrap_or(AST::Null);

                    match value {
                        AST::Number(n) => {
                            temp_ast.push(AST::Addition {
                                left: Box::new(AST::Number(n)),
                                right: Box::new(AST::Null),
                                line: current_line,
                            });
                        }

                        AST::Float(f) => {
                            temp_ast.push(AST::Addition {
                                left: Box::new(AST::Float(f)),
                                right: Box::new(AST::Null),
                                line: current_line,
                            });
                        }

                        AST::String(s) => {
                            temp_ast.push(AST::Addition {
                                left: Box::new(AST::String(s)),
                                right: Box::new(AST::Null),
                                line: current_line,
                            });
                        }

                        AST::LetDeclaration { name, value, line } => {
                            temp_ast.push(AST::LetDeclaration {
                                name,
                                value: Box::new(AST::Addition {
                                    left: value,
                                    right: Box::new(AST::Null),
                                    line,
                                }),
                                line,
                            });
                        }
                        
                        AST::Call { name, mut args, line } => {
                            let arg = args.pop().unwrap_or(AST::Null);

                            args.push(AST::Addition {
                                left: Box::new(arg),
                                right: Box::new(AST::Null),
                                line,
                            });

                            temp_ast.push(AST::Call {
                                name,
                                args,
                                line,
                            });
                        }

                        _ => {
                            return Err((format!("Expected a number, float, string, or let declaration before '+', got {:?}", value), current_line));
                        }
                    }
                }    

                Ok(Token::Minus) => {
                    let value = temp_ast.pop().unwrap_or(AST::Null);

                    match value {
                        AST::Number(n) => {
                            temp_ast.push(AST::Subtraction {
                                left: Box::new(AST::Number(n)),
                                right: Box::new(AST::Null),
                                line: current_line,
                            });
                        }

                        AST::Float(f) => {
                            temp_ast.push(AST::Subtraction {
                                left: Box::new(AST::Float(f)),
                                right: Box::new(AST::Null),
                                line: current_line,
                            });
                        }

                        AST::LetDeclaration { name, value, line } => {
                            temp_ast.push(AST::LetDeclaration {
                                name,
                                value: Box::new(AST::Subtraction {
                                    left: value,
                                    right: Box::new(AST::Null),
                                    line,
                                }),
                                line,
                            });
                        }

                        AST::Call { name, mut args, line } => {
                            let arg = args.pop().unwrap_or(AST::Null);

                            args.push(AST::Subtraction {
                                left: Box::new(arg),
                                right: Box::new(AST::Null),
                                line,
                            });

                            temp_ast.push(AST::Call {
                                name,
                                args,
                                line,
                            });
                        }

                        _ => {
                            return Err((format!("Expected a number, float, or let declaration before '-', got {:?}", value), current_line));
                        }
                    }
                }

                Ok(Token::Number) => {
                    let value = temp_ast.pop().unwrap_or(AST::Null);

                    match value {
                        AST::Call { name, mut args, line } => {
                            let arg = args.pop().unwrap_or(AST::Null);

                            match arg {
                                AST::Addition { left, right, line } => {
                                    args.push(AST::Addition {
                                        left,
                                        right: Box::new(AST::Number(lexer.slice().parse().unwrap())),
                                        line,
                                    });
                                }

                                AST::Subtraction { left, right, line } => {
                                    args.push(AST::Subtraction {
                                        left,
                                        right: Box::new(AST::Number(lexer.slice().parse().unwrap())),
                                        line,
                                    });
                                }

                                _ => {
                                    args.push(AST::Number(lexer.slice().parse().unwrap()));
                                }
                            }

                            temp_ast.push(AST::Call {
                                name,
                                args,
                                line,
                            });
                        }

                        AST::PropertyCall { object, property, args, line } => {
                            let mut args = args.clone();

                            args.push(AST::Number(lexer.slice().parse().unwrap()));

                            temp_ast.push(AST::PropertyCall {
                                object,
                                property,
                                args,
                                line,
                            });
                        }

                        AST::LetDeclaration { name, value, line } => {
                            if let AST::Addition { left, right, line } = *value {
                                temp_ast.push(AST::LetDeclaration {
                                    name,
                                    value: Box::new(AST::Addition {
                                        left,
                                        right: Box::new(AST::Number(lexer.slice().parse().unwrap())),
                                        line,
                                    }),
                                    line,
                                });
                            } else {
                                if let AST::Subtraction { left, right, line } = *value {
                                    temp_ast.push(AST::LetDeclaration {
                                        name,
                                        value: Box::new(AST::Subtraction {
                                            left,
                                            right: Box::new(AST::Number(lexer.slice().parse().unwrap())),
                                            line,
                                        }),
                                        line,
                                    });
                                } else {
                                    temp_ast.push(AST::LetDeclaration {
                                        name,
                                        value: Box::new(AST::Number(lexer.slice().parse().unwrap())),
                                        line,
                                    });
                                }
                            }
                        }

                        AST::Addition { left, right, line } => {
                            if let AST::Null = *right {
                                temp_ast.push(AST::Addition {
                                    left,
                                    right: Box::new(AST::Number(lexer.slice().parse().unwrap())),
                                    line,
                                });
                            } else {
                                return Err(("Expected a value before '+'".to_string(), current_line));
                            }
                        }

                        AST::Subtraction { left, right, line } => {
                            temp_ast.push(AST::Subtraction {
                                left,
                                right: Box::new(AST::Number(lexer.slice().parse().unwrap())),
                                line,
                            });
                        }

                        AST::IfStatement { condition, body, line } => {
                            if let AST::IsEqual { left, right, line } = condition.as_ref() {
                                if let AST::Null = **right {
                                    temp_ast.push(AST::IfStatement {
                                        condition: Box::new(AST::IsEqual {
                                            left: left.clone(),
                                            right: Box::new(AST::Number(lexer.slice().parse().unwrap())),
                                            line: *line,
                                        }),
                                        body,
                                        line: *line,
                                    });
                                } else {
                                    return Err(("Expected a value before '=='".to_string(), current_line));
                                }
                            } else {
                                if let AST::IsUnequal { left, right, line } = condition.as_ref() {
                                    if let AST::Null = **right {
                                        temp_ast.push(AST::IfStatement {
                                            condition: Box::new(AST::IsUnequal {
                                                left: left.clone(),
                                                right: Box::new(AST::Number(lexer.slice().parse().unwrap())),
                                                line: *line,
                                            }),
                                            body,
                                            line: *line,
                                        });
                                    } else {
                                        return Err(("Expected a value before '!='".to_string(), current_line));
                                    }
                                } else {
                                    temp_ast.push(AST::IfStatement {
                                        condition,
                                        body,
                                        line,
                                    });
        
                                    temp_ast.push(AST::Number(lexer.slice().parse().unwrap()));
                                }
                            }
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

                        AST::PropertyCall { object, property, args, line } => {
                            let mut args = args.clone();

                            args.push(AST::Boolean(lexer.slice() == "true"));

                            temp_ast.push(AST::PropertyCall {
                                object,
                                property,
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

                        AST::IfStatement { condition, body, line } => {
                            if let AST::IsEqual { left, right, line } = condition.as_ref() {
                                if let AST::Null = **right {
                                    temp_ast.push(AST::IfStatement {
                                        condition: Box::new(AST::IsEqual {
                                            left: left.clone(),
                                            right: Box::new(AST::Boolean(lexer.slice() == "true")),
                                            line: *line,
                                        }),
                                        body,
                                        line: *line,
                                    });
                                } else {
                                    return Err(("Expected a value before '=='".to_string(), current_line));
                                }
                            } else {
                                if let AST::IsUnequal { left, right, line } = condition.as_ref() {
                                    if let AST::Null = **right {
                                        temp_ast.push(AST::IfStatement {
                                            condition: Box::new(AST::IsUnequal {
                                                left: left.clone(),
                                                right: Box::new(AST::Boolean(lexer.slice() == "true")),
                                                line: *line,
                                            }),
                                            body,
                                            line: *line,
                                        });
                                    } else {
                                        return Err(("Expected a value before '!='".to_string(), current_line));
                                    }
                                } else {
                                    temp_ast.push(AST::IfStatement {
                                        condition,
                                        body,
                                        line,
                                    });
        
                                    temp_ast.push(AST::Boolean(lexer.slice() == "true"));
                                }
                            }
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
                            let arg = args.pop().unwrap_or(AST::Null);

                            match arg {
                                AST::Addition { left, right, line } => {
                                    args.push(AST::Addition {
                                        left,
                                        right: Box::new(AST::Float(lexer.slice().parse().unwrap())),
                                        line,
                                    });
                                }

                                AST::Subtraction { left, right, line } => {
                                    args.push(AST::Subtraction {
                                        left,
                                        right: Box::new(AST::Float(lexer.slice().parse().unwrap())),
                                        line,
                                    });
                                }

                                _ => {
                                    args.push(AST::Float(lexer.slice().parse().unwrap()));
                                }
                            }

                            temp_ast.push(AST::Call {
                                name,
                                args,
                                line,
                            });
                        }

                        AST::PropertyCall { object, property, args, line } => {
                            let mut args = args.clone();
    
                            args.push(AST::Float(lexer.slice().parse().unwrap()));
    
                            temp_ast.push(AST::PropertyCall {
                                object,
                                property,
                                args,
                                line,
                            });
                        }

                        AST::LetDeclaration { name, value, line } => {
                            if let AST::Addition { left, right, line } = *value {
                                temp_ast.push(AST::LetDeclaration {
                                    name,
                                    value: Box::new(AST::Addition {
                                        left,
                                        right: Box::new(AST::Float(lexer.slice().parse().unwrap())),
                                        line,
                                    }),
                                    line,
                                });
                            } else {
                                temp_ast.push(AST::LetDeclaration {
                                    name,
                                    value: Box::new(AST::Float(lexer.slice().parse().unwrap())),
                                    line,
                                });
                            }
                        }

                        AST::IfStatement { condition, body, line } => {
                            temp_ast.push(AST::IfStatement {
                                condition,
                                body,
                                line,
                            });

                            temp_ast.push(AST::Float(lexer.slice().parse().unwrap()));
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

                        AST::PropertyCall { object, property, args, line } => {
                            temp_ast.push(AST::PropertyCall {
                                object,
                                property,
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
                            return Err((format!("Expected a call or property call before ')', got {:?}", temp_ast.pop().unwrap_or(AST::Null)), current_line));
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

                            bodies_deep += 1;
                            body_starts = true;
                        }

                        AST::IfStatement { condition, body, line } => {
                            temp_ast.push(AST::IfStatement {
                                condition,
                                body,
                                line,
                            });

                            bodies_deep += 1;
                            body_starts = true;
                        }

                        AST::Identifer(name) => {
                            match temp_ast.pop() {
                                Some(AST::IfStatement { condition, body, line }) => {
                                    temp_ast.push(AST::IfStatement {
                                        condition: Box::new(AST::Exists {
                                            value: Box::new(AST::Identifer(name)),
                                            line,
                                        }),
                                        body,
                                        line,
                                    });

                                    bodies_deep += 1;
                                    body_starts = true;
                                }

                                Some(_) => {
                                    return Err(("Expected an if statement before '{'".to_string(), current_line));
                                }

                                None => {
                                    return Err(("Expected an if statement before '{'".to_string(), current_line));
                                }
                            }
                        }

                        _ => {
                            return Err(("Expected a function or if statement before '{'".to_string(), current_line));
                        }
                    }
                }

                Ok(Token::RBracket) => {
                    let value = ast.pop().unwrap_or(AST::Null);

                    match value {
                        AST::Function { name, args, body, line } => {
                            ast.push(AST::Function {
                                name,
                                args,
                                body,
                                line,
                            });

                            bodies_deep -= 1;
                        }

                        AST::IfStatement { condition, body, line } => {
                            ast.push(AST::IfStatement {
                                condition,
                                body,
                                line,
                            });

                            bodies_deep -= 1;
                        }

                        _ => {
                            return Err(("Expected a function or if statement before '}'".to_string(), current_line));
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
            dbg!(&temp_ast);
        }

        if bodies_deep > 0 && (!body_starts || bodies_deep > 1) {
            ast = handle_nested_ast(ast, temp_ast, current_line)?;
        } else {
            ast.append(&mut temp_ast);
        }

        if verbose {
            dbg!(&ast);
        }
    }

    if verbose {
        dbg!(&ast);
    }

    for item in ast.clone() {
        match item {
            AST::Import { file, as_, line } => {
                let result = eval(AST::Import { file, as_, line }, context);

                if result.is_err() {
                    return Err((result.err().unwrap(), line));
                }
            }

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

            AST::PropertyCall { object, property, args, line } => {
                let result = eval(AST::PropertyCall { object, property, args, line }, context);

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

            AST::IfStatement { condition, body, line } => {
                let result = eval(AST::IfStatement { condition, body, line }, context);

                if result.is_err() {
                    return Err((result.err().unwrap(), line));
                }
            }

            AST::Semicolon => {}

            _ => {
                if verbose {
                    println!("I'm not sure what to do with a {:?}", item);
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn let_str() {
        let mut context = crate::utils::create_context();
        let result = parse("let x = \"test\"", &mut context);

        assert_eq!(result, Ok(()));
    }


    #[test]
    fn let_number() {
        let mut context = crate::utils::create_context();
        let result = parse("let x = 10", &mut context);

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn let_boolean() {
        let mut context = crate::utils::create_context();
        let result = parse("let x = true", &mut context);

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn let_float() {
        let mut context = crate::utils::create_context();
        let result = parse("let x = 1.123", &mut context);

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn let_unknown_var() {
        let mut context = crate::utils::create_context();
        let result = parse("let x = y", &mut context);

        assert_eq!(result, Err(("Variable y not found".to_string(), 1)));
    }

    #[test]
    fn print_str() {
        let mut context = crate::utils::create_context();
        let result = parse("print(\"Hello, world!\")", &mut context);

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn print_number() {
        let mut context = crate::utils::create_context();
        let result = parse("print(10)", &mut context);

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn print_float() {
        let mut context = crate::utils::create_context();
        let result = parse("print(1.123)", &mut context);

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn print_boolean() {
        let mut context = crate::utils::create_context();
        let result = parse("print(true)", &mut context);

        assert_eq!(result, Ok(()));

    }

    #[test]
    fn print_var() {
        let mut context = crate::utils::create_context();
        parse("let x = 10", &mut context).unwrap();

        let result = parse("print(x)", &mut context);

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn print_unknown_var() {
        let mut context = crate::utils::create_context();
        let result = parse("print(x)", &mut context);

        assert_eq!(result, Ok(())); // cause prints Null
    }
}