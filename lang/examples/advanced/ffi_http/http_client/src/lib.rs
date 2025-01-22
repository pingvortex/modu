use reqwest::blocking::Client;

#[unsafe(no_mangle)]
pub extern "C" fn get(
    argc: std::ffi::c_int,
    argv: *const *const std::ffi::c_char
) -> *mut std::ffi::c_char {
    if argc != 1 {
        panic!("function 'get' requires and takes 1 arguments");
    }

    let args = unsafe {
        std::slice::from_raw_parts(argv, argc as usize)
    };

    let url = unsafe {
        std::ffi::CStr::from_ptr(args[0])
    };
    let url = url.to_str().unwrap();

    let client = Client::new();
    let res = client.get(url).send().unwrap();
    let body = res.text().unwrap();

    std::ffi::CString::new(body).unwrap().into_raw()
}