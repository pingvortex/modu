mod lexer;
mod ast;
mod parser;
mod eval;

use std::io::Write;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let action = &args[1];

    match action.as_str() {
        "run" => {
            let file = std::fs::read_to_string(&args[2]).unwrap();

            let context = &mut std::collections::HashMap::new();

            for line in file.lines() {
                parser::parse_line(line, context).unwrap();
            }  
        }

        "repl" => {
            println!("Modu REPL");

            let context = &mut std::collections::HashMap::new();

            loop {
                let mut input = String::new();

                print!("> ");
                std::io::stdout().flush().unwrap();

                std::io::stdin().read_line(&mut input).unwrap();

                parser::parse_line(&input, context).unwrap_or_else(|e| {
                    println!("Error: {}", e);
                    ast::AST::Null
                });
            }
        }

        _ => {
            println!("Invalid action");
        }
    }
}
