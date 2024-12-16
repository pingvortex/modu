
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum AST {
    Comment,

    LetDeclaration {
        name: Option<String>,
        value: Box<AST>,
        line: usize, // for error msgs
    },

    IfStatement {
        condition: Box<AST>,
        body: Vec<AST>,
        line: usize,
    },

    Import {
        file: Option<String>,
        as_: Option<String>,
        line: usize,
    },

    Object {
        properties: HashMap<String, AST>,
        line: usize,
    },

    PropertyAccess {
        object: Option<String>,
        property: Option<String>,
        line: usize,
    },

    PropertyCall {
        object: Option<String>,
        property: Option<String>,
        args: Vec<AST>,
        line: usize,
    },

    Call {
        name: String,
        args: Vec<AST>,
        line: usize,
    },

    Function {
        name: String,
        args: Vec<String>,
        body: Vec<AST>,
        line: usize,
    },

    IsEqual {
        left: Box<AST>,
        right: Box<AST>,
        line: usize,
    },

    IsUnequal {
        left: Box<AST>,
        right: Box<AST>,
        line: usize,
    },

    Addition {
        left: Box<AST>,
        right: Box<AST>,
        line: usize,
    },

    Subtraction {
        left: Box<AST>,
        right: Box<AST>,
        line: usize,
    },

    Identifer(String),

    Number(i64),

    String(String),

    Boolean(bool),

    Float(f64),

    Null,

    Comma,

    /*
    
    LParen,

    RParen,

    LBracket,

    RBracket,

    */

    Semicolon,
}

