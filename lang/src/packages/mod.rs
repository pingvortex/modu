mod math;

use crate::ast::AST;

pub fn get_package(name: &str) -> Option<AST> {
    match name {
        "math" => {
            Some(AST::Object {
                properties: math::get_object(),
                line: 0,
            })
        }
        _ => None
    }
}