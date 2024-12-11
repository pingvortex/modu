mod lexer;
mod ast;
mod parser;
mod eval;


fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let file = std::fs::read_to_string(&args[2]).unwrap();

    for line in file.lines() {
        let expr = parser::parse_call(line).unwrap();
        eval::eval(expr);
    }   
}
