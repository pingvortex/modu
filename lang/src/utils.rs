use std::collections::HashMap;

use crate::ast::AST;

pub fn is_reserved(name: &str) -> bool {
    match name {
        "let" | "fn" | "import" | "if" | "null" => true,
        _ => false,
    }
}

pub fn create_context() -> HashMap<String, AST> {
    let mut context = HashMap::new();

    context.insert(
        "print".to_string(), 
        AST::InternalFunction { 
            name: "print".to_string(), 
            args: vec!["value".to_string()], 
            call_fn: crate::internal::print
        }
    );

    context.insert(
        "exit".to_string(), 
        AST::InternalFunction { 
            name: "exit".to_string(), 
            args: vec![], 
            call_fn: crate::internal::exit,
        }
    );
    
    return context;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_reserved_test() {
        assert_eq!(is_reserved("let"), true);
        assert_eq!(is_reserved("fn"), true);
        assert_eq!(is_reserved("import"), true);
        assert_eq!(is_reserved("if"), true);
        assert_eq!(is_reserved("potato"), false);
    }

    #[test]
    fn create_context_test() {
        let context = create_context();

        assert_eq!(context.len(), 2);
        assert_eq!(context.contains_key("print"), true);
        assert_eq!(context.contains_key("exit"), true);
    }
}