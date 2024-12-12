pub fn is_reserved(name: &str) -> bool {
    match name {
        "let" | "print" | "exit" => true,
        _ => false,
    }
}