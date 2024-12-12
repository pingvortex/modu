mod lexer;
mod ast;
mod parser;
mod eval;
mod utils;

use std::io::Write;

use bat::{Input, PrettyPrinter};
use bat::line_range::{LineRanges, LineRange};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Usage: modu [run|repl] [file]");
        return;
    }

    let action = &args[1];

    match action.as_str() {
        "run" => {
            let file = std::fs::read_to_string(&args[2]).unwrap();

            let context = &mut std::collections::HashMap::new();

            let mut current_line = 0;

            for line in file.lines() {
                current_line += 1;

                parser::parse_line(line, context).unwrap_or_else(|e| {
                    println!("\n⚠️  {}", e);
                    println!("Traceback (most recent call last):"); 
                    println!("    File \"{}\", line {}", &args[2], current_line);

                    PrettyPrinter::new()
                        .language("rust")
                        .header(true)
                        .line_numbers(true)
                        .highlight(current_line)
                        .grid(true)
                        .input_file(std::path::Path::new(&args[2]))
                        .line_ranges(
                            LineRanges::from(vec![LineRange::from(&format!("{}:{}", current_line - 1, current_line + 1)).unwrap()])
                        )
                        .print()
                        .unwrap();
                    
                    std::process::exit(1);
                });
            }  
        }

        "repl" => {
            println!("Modu REPL");

            let context = &mut std::collections::HashMap::new();
            
            let mut current_line = 0;
            let mut history: Vec<String> = Vec::new();

            loop {
                current_line += 1;

                let mut input = String::new();

                print!("> ");
                std::io::stdout().flush().unwrap();

                std::io::stdin().read_line(&mut input).unwrap();

                history.push(input.clone());

                parser::parse_line(&input, context).unwrap_or_else(|e| {
                    println!("\n⚠️  {}", e);
                    println!("Traceback (most recent call last):");
                    println!("    File \"<stdin>\", line {}", current_line);

                    let joined = history.join("");
                    let bytes = joined.as_bytes();

                    PrettyPrinter::new()
                        .language("rust")
                        .header(true)
                        .line_numbers(true)
                        .highlight(current_line)
                        .grid(true)
                        .input_from_bytes(bytes)
                        .line_ranges(
                            LineRanges::from(vec![LineRange::from(&format!("{}:{}", current_line - 1, current_line + 1)).unwrap()])
                        )
                        .print()
                        .unwrap();

                    println!();

                    ast::AST::Null
                });
            }
        }

        _ => {
            println!("Invalid action");
        }
    }
}
