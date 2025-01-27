# Foreign Function Interface (FFI)
> Disabled on the server due to security >:D

⚠️ Can only take strings (or variables that are converted to strings) as arguments

Using FFI is actually really simple, just import a **.dll/.so/.dylib** file and u can run its functions with ffi.call, here is an example:
```rust
import "ffi" as ffi;

// Note that .so is the shared library extension on Linux
// On windows it would be .dll, and on MacOS it would be .dylib
// In actal code you would have to differentiate using os.name 
// (returns windows/linux/macos/unknown)
// For info on the OS package see the "OS Lib" page
ffi.call("./libffi_test.so", "hello_world");

// Output:
//
// Hello, World
```

This is the **hello_world** function, written as a rust lib:
```rust
#[unsafe(no_mangle)]
pub extern "C" fn hello_world() {
    println!("Hello, World!");
}
```

Note: i am using rust cause i prefer that, you can write the libraries in any programming that exports to C-Style libraries. \
Here are some examples:
- C (of course you can use C)
- Go (using CGO)
- Python (using ctypes)


## Arguments
To use arguments call the function like **ffi.call(path, function, arg1, arg2, ...)**

Here is an example:
```rust
import "ffi" as ffi;

// FFI currently only accept string args, 
// so we have to parse it to int in the lib

// You can either just provide a string with the number, 
// or use the str() function

// It does not make any diffrence which one you use, 
// but the str(int) can be used for variable numbers
print(ffi.call("./libffi_test.so", "add", str(5), "2"));

// Output:
//
// 7
```

As you can see, we have to use string arguments, any other will cause errors.

Here is the code for the library:
```rust
#[unsafe(no_mangle)]
pub extern "C" fn add(
    // Amount of args in argv
    argc: std::ffi::c_int,
    // We are using char cause we are passing strings, 
    // we will turn this to int later
    argv: *const *const std::ffi::c_char
) -> i32 {
    // We can check argc to enforce arg requirements/limints
    if argc != 2 {
        panic!("add requires 2 arguments");
    }

    // Turn argv into an vec containing our args
    let args = unsafe {
        std::slice::from_raw_parts(argv, argc as usize)
    };

    // Parse the pointers to strings
    let num1 = unsafe {
        std::ffi::CString::from_raw(args[0] as *mut std::ffi::c_char)
    };
    // Then we finally parse them to numbers
    let num1 = num1.to_str().unwrap().parse::<i32>().unwrap();

    let num2 = unsafe {
        std::ffi::CString::from_raw(args[1] as *mut std::ffi::c_char)
    };
    let num2 = num2.to_str().unwrap().parse::<i32>().unwrap();

    num1 + num2
}
```