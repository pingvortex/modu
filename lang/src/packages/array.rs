use std::collections::HashMap;
use crate::ast::AST;
use crate::eval::eval;

pub fn new(_: Vec<AST>, _: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let mut obj: HashMap<String, AST> = HashMap::new();

    obj.insert(
        "length".to_string(),
        AST::Number(0)
    );

    Ok((AST::Object { properties: obj, line: 0 }, AST::Null))
}

pub fn at(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let arr = eval(args[0].clone(), context)?;
    let index = eval(args[1].clone(), context)?;
    
    match (arr, index) {
        (AST::Object { properties: obj, line: _ }, AST::Number(i)) => {
            let itemornone = obj.get(&i.to_string());
            if let None = itemornone {
                return Err("no such element at that index".to_string());
            }
            let item = itemornone.unwrap();
            Ok((item.clone(), AST::Null))
        },

        _ => Err("at() expects an array and a number".to_string())
    }
}

pub fn push(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let arr = eval(args[0].clone(), context)?;
    let item = eval(args[1].clone(), context)?;

    match arr {
        AST::Object { properties: mut obj, line: _ } => {
            let objclone = obj.clone();
            let lenornone = objclone.get(&"length".to_string());

            if let None = lenornone {
                return Err("corrupted array".to_string());
            }

            let len = lenornone.unwrap();

            match len {
                AST::Number(length) => {
                    obj.insert(
                        length.to_string(),
                        item
                    );

                    obj.insert("length".to_string(), AST::Number(length+1));

                    Ok((AST::Null, AST::Null))
                }

                _ => Err("corrupted array".to_string())
            }
        }

        _ => Err("push() expects an array and a value".to_string())
    }
}

pub fn pop(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let arr = eval(args[0].clone(), context)?;

    match arr {
        AST::Object { properties: mut obj, line: _ } => {
            let objclone = obj.clone();
            let lenornone = objclone.get(&"length".to_string());

            if let None = lenornone {
                return Err("corrupted array".to_string());
            }

            let len = lenornone.unwrap();

            match len {
                AST::Number(length) => {
                    if *length < 1 {
                        return Err("empty array".to_string());
                    }

                    let lastornone = objclone.get(&(length-1).to_string());

                    if let None = lastornone {
                        return Err("corrupted array".to_string());
                    }

                    let last = lastornone.unwrap();

                    obj.remove(&(length-1).to_string());
                    obj.insert("length".to_string(), AST::Number(length-1));

                    Ok((last.clone(), AST::Null))
                }
                
                _ => Err("corrupted array".to_string())
            }
        }

        _ => Err("pop() expects an array".to_string())
    }
}

pub fn shift(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let arr = eval(args[0].clone(), context)?;

    match arr {
        AST::Object { properties: mut obj, line: _ } => {
            let objclone = obj.clone();
            let lenornone = objclone.get(&"length".to_string());

            if let None = lenornone {
                return Err("corrupted array".to_string());
            }

            let len = lenornone.unwrap();

            match len {
                AST::Number(length) => {
                    if *length < 1 {
                        return Err("empty array".to_string());
                    }

                    let firstornone = objclone.get(&0.to_string());

                    if let None = firstornone {
                        return Err("corrupted array".to_string());
                    }

                    let first = firstornone.unwrap();

                    let mut elems: Vec<AST> = vec![];

                    if *length > 1 {
                        for i in 1..*length {
                            let elemornone = objclone.get(&i.to_string());

                            if let None = elemornone {
                                return Err("corrupted array".to_string());
                            }

                            let elem = elemornone.unwrap();

                            elems.push(elem.clone());
                        }
                    }

                    obj.clear();

                    if *length > 1 {
                        let mut i: i64 = 0;
                        for elem in elems {
                            obj.insert(i.to_string(), elem);
                            i += 1;
                        }
                    }
                    obj.insert("length".to_string(), AST::Number(length-1));

                    Ok((first.clone(), AST::Null))
                }

                _ => Err("corrupted array".to_string())
            }
        }

        _ => Err("pop() expects an array".to_string())
    }
}

pub fn unshift(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let arr = eval(args[0].clone(), context)?;
    let item = eval(args[1].clone(), context)?;

    match arr {
        AST::Object { properties: mut obj, line: _ } => {
            let objclone = obj.clone();

            let lenornone = objclone.get(&"length".to_string());

            if let None = lenornone {
                return Err("corrupted array".to_string());
            }

            let len = lenornone.unwrap();

            match len {
                AST::Number(length) => {
                    let mut elems: Vec<AST> = vec![];

                    if *length > 0 {
                        for i in 0..*length {
                            let elemornone = objclone.get(&i.to_string());

                            if let None = elemornone {
                                return Err("corrupted array".to_string());
                            }

                            let elem = elemornone.unwrap();

                            elems.push(elem.clone());
                        }
                    }

                    obj.clear();

                    obj.insert(0.to_string(), item);

                    if *length > 0 {
                        let mut i: i64 = 1;
                        for elem in elems {
                            obj.insert(i.to_string(), elem);
                            i += 1;
                        }
                    }
                    obj.insert("length".to_string(), AST::Number(length+1));

                    Ok((AST::Null, AST::Null))
                }

                _ => Err("corrupted array".to_string())
            }
        }

        _ => Err("pop() expects an array".to_string())
    }
}

pub fn get_object() -> HashMap<String, AST> {
    let mut object = HashMap::new();

    object.insert(
        "new".to_string(),
        AST::InternalFunction {
            name: "new".to_string(),
            args: vec![],
            call_fn: new
        }
    );
    object.insert(
        "at".to_string(),
        AST::InternalFunction {
            name: "at".to_string(),
            args: vec!["arr".to_string(), "index".to_string()],
            call_fn: at
        }
    );
    object.insert(
        "push".to_string(),
        AST::InternalFunction {
            name: "push".to_string(),
            args: vec!["arr".to_string(), "item".to_string()],
            call_fn: push
        }
    );
    object.insert(
        "pop".to_string(),
        AST::InternalFunction {
            name: "pop".to_string(),
            args: vec!["arr".to_string()],
            call_fn: pop
        }
    );
    object.insert(
        "shift".to_string(),
        AST::InternalFunction {
            name: "shift".to_string(),
            args: vec!["arr".to_string()],
            call_fn: shift
        }
    );
    object.insert(
        "unshift".to_string(),
        AST::InternalFunction {
            name: "unshift".to_string(),
            args: vec!["arr".to_string(), "item".to_string()],
            call_fn: unshift
        }
    );

    object
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_object_test() {
        let object = get_object();

        assert_eq!(object.len(), 6);
    }
}