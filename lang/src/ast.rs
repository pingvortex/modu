
#[derive(Debug, PartialEq, Clone)]
pub enum AST {
    Comment,

    LetDeclaration {
        name: Option<String>,
        value: Box<AST>,
        line: usize,
    },

    Identifer(String),

    Number(i64),

    String(String),

    Boolean(bool),

    Float(f64),

    Null,
    
    LParen(Box<AST>),

    RParen(Box<AST>),

    Semiclon,

    Call {
        name: String,
        args: Vec<AST>,
        line: usize,
    },
}

