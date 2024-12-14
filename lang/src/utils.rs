pub fn is_reserved(name: &str) -> bool {
    match name {
        "let" | "fn" | "import" | "if" => true,
        _ => false,
    }
}