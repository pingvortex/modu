
#[derive(Debug, PartialEq, Clone)]
pub enum AST {
    LetDeclaration {
        name: Option<String>,
        value: Box<AST>,
    },

    Identifer(String),

    Number(i64),

    String(String),

    Null,
    
    LParen(Box<AST>),

    RParen(Box<AST>),

    Call {
        name: String,
        args: Vec<AST>,
    },
}

