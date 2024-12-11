use crate::ast::AST;
use crate::lexer::{Token, LexingError};

use logos::Logos;

pub fn parse_call(input: &str) -> Result<AST, String> {
    let mut lexer = Token::lexer(input);
    let mut ast = Vec::new();

    while let Some(token) = lexer.next() {
        match token {
            Ok(Token::Identifer) => {
                ast.push(AST::Identifer(lexer.slice().to_string()));
            }

            Ok(Token::LParen) => {
                if let Some(AST::Identifer(name)) = ast.pop() {
                    ast.push(AST::Call {
                        name,
                        args: Vec::new(),
                    })
                } else {
                    return Err("Expected an identifier before '(...)'".to_string());
                }
            }

            Ok(Token::String) => {
                if let Some(AST::Call { name, args }) = ast.pop() {
                    ast.push(AST::Call {
                        name,
                        args: vec![AST::String(lexer.slice().to_string())],
                    });
                } else {
                    return Err("Expected a call before a string".to_string());
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

    if ast.len() == 1 {
        Ok(ast.pop().unwrap())
    } else {
        Err("Expected a single expression".to_string())
    }
}
