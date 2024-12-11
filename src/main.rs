mod lexer;
mod ast;
mod parser;
mod eval;


fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    //let file = std::fs::read_to_string(&args[2]).unwrap();

    let expr = parser::parse_call("print(\"Hello, world!\")").unwrap();
    eval::eval(expr);
}
