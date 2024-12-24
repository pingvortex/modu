use std::collections::HashMap;
use rand;

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

pub fn abs(args: Vec<AST>, _: &mut HashMap<String, AST>) -> Result<AST, String> {
    match eval(args[0].clone(), &mut HashMap::new()) {
        Ok(AST::Number(a)) => {
            return Ok(AST::Number(a.abs()));
        }

        Ok(AST::Float(a)) => {
            return Ok(AST::Float(a.abs()));
        }

        _ => {
            return Err("abs requires a number".to_string());
        }
    }
}

pub fn sqrt(args: Vec<AST>, _: &mut HashMap<String, AST>) -> Result<AST, String> {
    match eval(args[0].clone(), &mut HashMap::new()) {
        Ok(AST::Number(a)) => {
            if a < 0 {
                return Err("sqrt requires a positive number".to_string());
            }

            return Ok(AST::Float((a as f64).sqrt()));
        }

        Ok(AST::Float(a)) => {
            if a < 0.0 {
                return Err("sqrt requires a positive number".to_string());
            }

            return Ok(AST::Float(a.sqrt()));
        }

        _ => {
            return Err("sqrt requires a number".to_string());
        }
    }
}

pub fn pow(args: Vec<AST>, _: &mut HashMap<String, AST>) -> Result<AST, String> {
    match eval(args[0].clone(), &mut HashMap::new()) {
        Ok(AST::Number(a)) => {
            match eval(args[1].clone(), &mut HashMap::new()) {
                Ok(AST::Number(b)) => {
                    return Ok(AST::Number(a.pow(b as u32)));
                }

                Ok(AST::Float(b)) => {
                    return Ok(AST::Float((a as f64).powf(b)));
                }

                _ => {
                    return Err("pow requires a number".to_string());
                }
            }
        }

        Ok(AST::Float(a)) => {
            match eval(args[1].clone(), &mut HashMap::new()) {
                Ok(AST::Number(b)) => {
                    return Ok(AST::Float(a.powf(b as f64)));
                }

                Ok(AST::Float(b)) => {
                    return Ok(AST::Float(a.powf(b)));
                }

                _ => {
                    return Err("pow requires a number".to_string());
                }
            }
        }

        _ => {
            return Err("pow requires a number".to_string());
        }
    }
}

pub fn floor(args: Vec<AST>, _: &mut HashMap<String, AST>) -> Result<AST, String> {
    match eval(args[0].clone(), &mut HashMap::new()) {
        Ok(AST::Number(a)) => {
            return Ok(AST::Number(a));
        }

        Ok(AST::Float(a)) => {
            return Ok(AST::Number(a.floor() as i64));
        }

        _ => {
            return Err("floor requires a number".to_string());
        }
    }
}

pub fn ceil(args: Vec<AST>, _: &mut HashMap<String, AST>) -> Result<AST, String> {
    match eval(args[0].clone(), &mut HashMap::new()) {
        Ok(AST::Number(a)) => {
            return Ok(AST::Number(a));
        }

        Ok(AST::Float(a)) => {
            return Ok(AST::Number(a.ceil() as i64));
        }

        _ => {
            return Err("ceil requires a number".to_string());
        }
    }
}

pub fn random(args: Vec<AST>, _: &mut HashMap<String, AST>) -> Result<AST, String> {
    return Ok(AST::Float(rand::random()));
}

pub fn random_int(args: Vec<AST>, _: &mut HashMap<String, AST>) -> Result<AST, String> {
    return Ok(AST::Number(rand::random()));
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

    objects.insert(
        "abs".to_string(), 
        AST::InternalFunction {
            name: "abs".to_string(),
            args: vec!["a".to_string()],
            call_fn: abs,
        }
    );

    objects.insert(
        "sqrt".to_string(), 
        AST::InternalFunction {
            name: "sqrt".to_string(),
            args: vec!["a".to_string()],
            call_fn: sqrt,
        }
    );

    objects.insert(
        "pow".to_string(), 
        AST::InternalFunction {
            name: "pow".to_string(),
            args: vec!["a".to_string(), "b".to_string()],
            call_fn: pow,
        }
    );

    objects.insert(
        "floor".to_string(), 
        AST::InternalFunction {
            name: "floor".to_string(),
            args: vec!["a".to_string()],
            call_fn: floor,
        }
    );

    objects.insert(
        "ceil".to_string(), 
        AST::InternalFunction {
            name: "ceil".to_string(),
            args: vec!["a".to_string()],
            call_fn: ceil,
        }
    );

    objects.insert(
        "random".to_string(),
        AST::InternalFunction {
            name: "random".to_string(),
            args: vec![],
            call_fn: random,
        }
    );

    objects.insert(
        "random_int".to_string(),
        AST::InternalFunction {
            name: "random_int".to_string(),
            args: vec![],
            call_fn: random_int,
        }
    );

    objects.insert(
        "pi".to_string(),
        AST::Float(std::f64::consts::PI)
    );

    objects
}

#[cfg(test)]
mod tests {
    use std::hash::Hash;

    use super::*;

    #[test]
    fn get_object_test() {
        let object = get_object();

        assert_eq!(object.len(), 9);
        assert_eq!(object.contains_key("div"), true);
    }

    #[test]
    fn div_test() {
        let mut context = HashMap::new();
        let args = vec![AST::Number(10), AST::Number(2)];
        let result = div(args, &mut context).unwrap();

        match result {
            AST::Number(a) => {
                assert_eq!(a, 5);
            }
            _ => panic!("Expected AST::Number")
        }
    }

    #[test]
    fn abs_test() {
        let mut context = HashMap::new();
        let args = vec![AST::Number(-10)];

        match abs(args, &mut context).unwrap() {
            AST::Number(a) => {
                assert_eq!(a, 10);
            }
            _ => panic!("Expected AST::Number")
        }
    }

    #[test]
    fn sqrt_test() {
        let mut context = HashMap::new();
        let args = vec![AST::Number(9)];

        match sqrt(args, &mut context).unwrap() {
            AST::Float(a) => {
                assert_eq!(a, 3.0);
            }

            _ => panic!("Expected AST::Float")
        }
    }

    #[test]
    fn pow_test() {
        let mut context = HashMap::new();
        let args = vec![AST::Number(2), AST::Number(3)];

        match pow(args, &mut context).unwrap() {
            AST::Number(a) => {
                assert_eq!(a, 8);
            }

            _ => panic!("Expected AST::Number")
        }
    }

    #[test]
    fn floor_test() {
        let mut context = HashMap::new();
        let args = vec![AST::Float(3.14)];

        match floor(args, &mut context).unwrap() {
            AST::Number(a) => {
                assert_eq!(a, 3);
            }

            _ => panic!("Expected AST::Number")
        }
    }

    #[test]
    fn ceil_test() {
        let mut context = HashMap::new();
        let args = vec![AST::Float(3.14)];

        match ceil(args, &mut context).unwrap() {
            AST::Number(a) => {
                assert_eq!(a, 4);
            }

            _ => panic!("Expected AST::Number")
        }
    }

    #[test]
    fn pi_test() {
        let object = get_object();

        match object.get("pi").unwrap() {
            AST::Float(a) => {
                assert_eq!(a, &std::f64::consts::PI);
            }

            _ => panic!("Expected AST::Float")
        }
    }

    #[test]
    fn div_by_zero() {
        let mut context = HashMap::new();
        let args = vec![AST::Number(10), AST::Number(0)];

        match div(args, &mut context) {
            Err(e) => {
                assert_eq!(e, "cannot divide by zero");
            }

            _ => panic!("Expected Err")
        }
    }
}