
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
    
    LParen,

    RParen,

    LBracket,

    RBracket,

    Semicolon,

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
}

