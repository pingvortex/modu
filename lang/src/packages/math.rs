use std::collections::HashMap;

use crate::ast::AST;
use crate::eval::eval;

pub fn div(args: Vec<AST>, _: &mut HashMap<String, AST>) -> Result<AST, String> {
    match (eval(args[0].clone(), &mut HashMap::new()), eval(args[1].clone(), &mut HashMap::new())) {
        (Ok(AST::Number(a)), Ok(AST::Number(b))) => {
            if b == 0 {
                return Err("cannot divide by zero".to_string());
            }

            let result = a as f64 / b as f64;

            if result.fract() == 0.0 {
                return Ok(AST::Number(result as i64));
            }

            return Ok(AST::Float(result));
        }

        (Ok(AST::Float(a)), Ok(AST::Float(b))) => {
            if b == 0.0 {
                return Err("cannot divide by zero".to_string());
            }

            return Ok(AST::Float(a / b));
        }

        (Ok(AST::Float(a)), Ok(AST::Number(b))) => {
            if b == 0 {
                return Err("cannot divide by zero".to_string());
            }

            return Ok(AST::Float(a / b as f64));
        }

        (Ok(AST::Number(a)), Ok(AST::Float(b))) => {
            if b == 0.0 {
                return Err("cannot divide by zero".to_string());
            }

            return Ok(AST::Float(a as f64 / b));
        }

        _ => {
            return Err("div requires 2 numbers".to_string());
        }
    }
}

pub fn get_object() -> HashMap<String, AST> {
    let mut objects = HashMap::new();

    objects.insert(
        "div".to_string(), 
        AST::InternalFunction {
            name: "div".to_string(),
            args: vec!["a".to_string(), "b".to_string()],
            call_fn: div,
        }
    );

    objects
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_object_test() {
        let object = get_object();

        assert_eq!(object.len(), 1);
        assert_eq!(object.contains_key("div"), true);
    }
}