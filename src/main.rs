mod lexer;
mod ast;
mod parser;
mod eval;


fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let file = std::fs::read_to_string(&args[2]).unwrap();

    let context = &mut std::collections::HashMap::new();

    for line in file.lines() {
        parser::parse_line(line, context).unwrap();
    }   
}
