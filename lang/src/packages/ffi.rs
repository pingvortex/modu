use std::collections::HashMap;
use libffi::high::Arg;

use crate::ast::AST;
use crate::eval::eval;

pub fn call(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<AST, String> {
    // (path_to_lib, function_name, arg1, arg2, ...)

    if args.len() < 2 {
        return Err("ffi.call requires at least 2 arguments".to_string());
    }

    let path = match eval(args[0].clone(), context) {
        Ok(AST::String(v)) => v,

        _ => return Err("ffi.call first argument must be a string".to_string()),
    };

    let name = match eval(args[1].clone(), context) {
        Ok(AST::String(v)) => v,

        _ => return Err("ffi.call second argument must be a string".to_string()),
    };

    unsafe {
        let lib = match libloading::Library::new(path) {
            Ok(lib) => lib,
            Err(e) => return Err(format!("Failed to load library: {}", e)),
        };

        let func_ptr: libloading::Symbol<*mut std::ffi::c_void> = match lib.get(name.as_bytes()) {
            Ok(func) => func,
            Err(e) => return Err(format!("Failed to load function: {}", e)),
        };

        let mut ffi_args = Vec::new();
        let mut cstrings = Vec::new();

        for arg in args.iter().skip(2) {
            match arg {
                AST::Number(v) => {
                    // me when temp value freed error if i put this inside the push as a normal person would do
                    let send_help = *v as std::ffi::c_int;

                    ffi_args.push(Arg::new(&send_help));
                }

                AST::String(v) => {
                    let c_str = std::ffi::CString::new(v.replace("\"", "")).unwrap();
                    cstrings.push(c_str);
                    ffi_args.push(Arg::new(&(cstrings.last().unwrap().as_ptr() as *const std::ffi::c_void)));
                }

                _ => return Err("ffi.call arguments must be numbers or strings".to_string()),
            };
        }

        let ret_type = libffi::middle::Type::pointer();

        let result_ptr: i32 = libffi::high::call::call(
            libffi::high::call::CodePtr::from_ptr(*func_ptr),
            ffi_args.as_slice(),
        );

        Ok(AST::Null)
    }
}

pub fn get_object() -> HashMap<String, AST> {
	let mut object = HashMap::new();

	object.insert(
        "call".to_string(),
        AST::InternalFunction {
            name: "call".to_string(),
            args: vec!["__args__".to_string()],
            call_fn: call,
        }
    );

	object
}