mod math;
mod file;

use crate::ast::AST;

pub fn get_package(name: &str) -> Option<AST> {
    match name {
        "math" => {
            Some(AST::Object {
                properties: math::get_object(),
                line: 0,
            })
        }

        "file" => {
            Some(AST::Object {
                properties: file::get_object(),
                line: 0,
            })
        }

        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_math_package() {
        let math = get_package("math").unwrap();
        match math {
            AST::Object { properties, line: _ } => {
                assert_eq!(properties.len(), 10);
                assert_eq!(properties.contains_key("div"), true);
            }
            _ => panic!("Expected AST::Object")
        }
    }

    #[test]
    fn get_file_package() {
        let file = get_package("file").unwrap();

        match file {
            AST::Object { properties, line: _ } => {
                assert_eq!(properties.len(), 3);
                assert_eq!(properties.contains_key("write_append"), true);
            }

            _ => panic!("Expected AST::Object")
        }
    }

    #[test]
    fn get_unknown_package() {
        let unknown = get_package("unknown");
        assert_eq!(unknown, None);
    }
}