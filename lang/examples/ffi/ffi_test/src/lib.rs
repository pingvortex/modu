#[unsafe(no_mangle)]
pub extern "C" fn add(
    argc: std::ffi::c_int,
    argv: *const *const std::ffi::c_char
) -> i32 {
    if argc != 2 {
        panic!("add requires 2 arguments");
    }

    let args = unsafe {
        std::slice::from_raw_parts(argv, argc as usize)
    };

    // modu ffi cant have numbers as args cause shit broke
    let num1 = unsafe {
        std::ffi::CString::from_raw(args[0] as *mut std::ffi::c_char)
    };
    let num1 = num1.to_str().unwrap().parse::<i32>().unwrap();

    let num2 = unsafe {
        std::ffi::CString::from_raw(args[1] as *mut std::ffi::c_char)
    };
    let num2 = num2.to_str().unwrap().parse::<i32>().unwrap();

    num1 + num2
}

#[unsafe(no_mangle)]
pub extern "C" fn a(
    argc: std::ffi::c_int,
    argv: *const *const std::ffi::c_char
) -> i32 {
    if argc != 1 {
        panic!("a requires 1 argument");
    }

    let args = unsafe {
        std::slice::from_raw_parts(argv, argc as usize)
    };

    let a = unsafe {
        std::ffi::CString::from_raw(args[0] as *mut std::ffi::c_char)
    };
    let a = a.to_str().unwrap();

    a.parse().unwrap()   
}

#[unsafe(no_mangle)]
pub extern "C" fn one() -> i64 {
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn string() -> *mut std::ffi::c_char {
    std::ffi::CString::new("Hello, World!").unwrap().into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn print(
    argc: std::ffi::c_int,
    argv: *const *const std::ffi::c_char
) {
    if argc != 1 {
        panic!("print requires 1 argument");
    }

    let args = unsafe {
        std::slice::from_raw_parts(argv, argc as usize)
    };

    let str = unsafe {
        std::ffi::CStr::from_ptr(args[0])
    };

    println!("{}", str.to_str().unwrap());
}

#[unsafe(no_mangle)]
pub extern "C" fn print2(
    argc: std::ffi::c_int,
    argv: *const *const std::ffi::c_char
) {
    if argc != 2 {
        panic!("print requires 2 arguments");
    }

    let args = unsafe {
        std::slice::from_raw_parts(argv, argc as usize)
    };

    let str = unsafe {
        std::ffi::CStr::from_ptr(args[0])
    };

    let str2 = unsafe {
        std::ffi::CStr::from_ptr(args[1])
    };

    print!("{}", str.to_str().unwrap());
    print!("{}", str2.to_str().unwrap());
    println!();
}


#[unsafe(no_mangle)]
pub extern "C" fn hello_world() {
    println!("Hello, World!");
}