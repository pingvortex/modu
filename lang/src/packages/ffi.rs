use std::collections::HashMap;
use crate::ast::AST;
use crate::eval::eval;

pub fn call(mut args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<AST, String> {
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

        let func: libloading::Symbol<unsafe extern "C" fn(
            argc: std::ffi::c_int,
            argv: *mut std::ffi::c_void
        ) -> *mut std::ffi::c_void> 
            = match lib.get(name.as_bytes()) {
                Ok(func) => func,
                Err(e) => return Err(format!("Failed to load function: {}", e)),
            };

        let mut args_ptr: Vec<*mut std::ffi::c_void> = Vec::new();

        args.remove(0);
        args.remove(0);

        for arg in args {
            match arg {
                AST::Number(v) => {
                    args_ptr.push(v as *mut std::ffi::c_void);
                }

                AST::String(v) => {
                    let c_str = std::ffi::CString::new(v.replace("\"", "")).unwrap();
                    args_ptr.push(c_str.into_raw() as *mut std::ffi::c_void);
                }

                _ => return Err("ffi.call arguments must be numbers or strings".to_string()),
            };
        }

        let result_ptr = func(
            args_ptr.len() as std::ffi::c_int,
            args_ptr.as_mut_ptr() as *mut std::ffi::c_void
        );

        if result_ptr.is_null() {
            return Ok(AST::Null);
        };

        if (result_ptr as i64) <= i32::MAX as i64 && (result_ptr as i64) >= i32::MIN as i64 {
            return Ok(AST::Number(result_ptr as i64));
        } else {
            let str = std::ffi::CStr::from_ptr(result_ptr as *const _);
            return Ok(AST::String(str.to_string_lossy().into_owned()))
        }
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