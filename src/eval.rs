use crate::ast::AST;

pub fn eval(expr: AST) {
    match expr {
        AST::Call { name, args } => {
            match name.as_str() {
                "print" => {
                    if let AST::String(s) = args[0].clone() {
                        println!("{}", s.replace("\"", ""));
                    }
                }

                _ => {
                    println!("Unknown function: {}", name);
                }
            }
        }

        _ => {
            println!("Unknown expression: {:?}", expr);
        }
    }
}