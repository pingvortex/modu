
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum AST {
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

    Return {
        value: Box<AST>,
        line: usize,
    },

    InternalFunction {
        name: String,
        args: Vec<String>,
        call_fn: fn(Vec<AST>, &mut HashMap<String, AST>) -> Result<AST, String>,
    },

    Exists {
        value: Box<AST>,
        line: usize,
    },

    IsEqual {
        left: Box<AST>,
        right: Box<AST>,
        line: usize,
    },

    LessThan {
        left: Box<AST>,
        right: Box<AST>,
        line: usize,
    },

    GreaterThan {
        left: Box<AST>,
        right: Box<AST>,
        line: usize,
    },

    LessThanOrEqual {
        left: Box<AST>,
        right: Box<AST>,
        line: usize,
    },

    GreaterThanOrEqual {
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

    Semicolon,

    Lparen,

    Rparen,

    RBracket,

    Comma,

    Dot,

    Minus,

    Plus,
}


impl std::fmt::Display for AST {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            // TODO: Implement more
            AST::String(s) => { 
                let s = s.replace("\\t", "\t")
                    .replace("\\n", "\n")
                    .replace("\\r", "\r")
                    .replace("\\\"", "\"")
                    .replace("\\\\", "\\")
                    .replace("\"", "")
                    .replace("'", "");

                write!(f, "{}", s)
            },
            AST::Number(n) => write!(f, "{}", n),
            AST::Float(n) => write!(f, "{}", n),
            AST::Boolean(b) => write!(f, "{}", b),
            AST::Null => write!(f, "null"),

            AST::Object { properties, line: _ } => {
                write!(f, "{{ ")?;

                let built_in_funcs = vec!["set"];

                if properties.len() == 0 || properties.len() - built_in_funcs.len() <= 0 {
                    write!(f, "}}")?;
                } else {
                    let mut str = String::new();

                    for (key, value) in properties {
                        if built_in_funcs.contains(&key.as_str()) {
                            continue;
                        }

                        let fixed_for_str = match value {
                            // cause strings in obj should have ""
                            AST::String(s) => format!("\"{}\"", s),
                            _ => format!("{}", value),
                        };

                        str.push_str(&format!("{}: {}, ", key, fixed_for_str));
                    }

                    if str.len() > 0 {
                        write!(f, "{}", &str[..str.len() - 2])?;
                    }


                    write!(f, " }}")?;
                }
                
                Ok(())
            }

            _ => write!(f, "{:?}", self),
        }
    }
}