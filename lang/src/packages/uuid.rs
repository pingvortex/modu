use std::collections::HashMap;
use crate::ast::AST;
use uuid;

pub fn v4(_: Vec<AST>, _: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    Ok((AST::String(uuid::Uuid::new_v4().to_string()), AST::Null))
}

pub fn get_object() -> HashMap<String, AST> {
    let mut object = HashMap::new();

    object.insert(
        "v4".to_string(),
        AST::InternalFunction {
            name: "v4".to_string(),
            args: vec![],
            call_fn: v4
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

        assert_eq!(object.len(), 1);
    }
}