pub fn is_reserved(name: &str) -> bool {
    match name {
        "let" | "fn" | "import" | "if" => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_reserved_test() {
        assert_eq!(is_reserved("let"), true);
        assert_eq!(is_reserved("fn"), true);
        assert_eq!(is_reserved("import"), true);
        assert_eq!(is_reserved("if"), true);
        assert_eq!(is_reserved("potato"), false);
    }
}