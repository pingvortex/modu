
#[derive(Debug, PartialEq, Clone)]
pub enum AST {
    Comment,

    LetDeclaration {
        name: Option<String>,
        value: Box<AST>,
    },

    Identifer(String),

    Number(i64),

    String(String),

    Boolean(bool),

    Float(f64),

    Null,
    
    LParen(Box<AST>),

    RParen(Box<AST>),

    Call {
        name: String,
        args: Vec<AST>,
    },
}

