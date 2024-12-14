mod lexer;
mod ast;
mod parser;
mod eval;
mod utils;

use std::io::Write;

use bat::PrettyPrinter;
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
            let file: String = std::fs::read_to_string(&args[2]).unwrap();

            let context = &mut std::collections::HashMap::new();

            parser::parse(&file, context).unwrap_or_else(|e| {
                println!("\n⚠️  {}", e.0);
                println!("Traceback (most recent call last):"); 
                println!("    File \"{}\", line {}", &args[2], e.1);

                PrettyPrinter::new()
                    .language("rust")
                    .header(true)
                    .line_numbers(true)
                    .highlight(e.1)
                    .grid(true)
                    .input_file(std::path::Path::new(&args[2]))
                    .line_ranges(
                        LineRanges::from(vec![LineRange::from(&format!("{}:{}", e.1 - 1, e.1 + 1)).unwrap()])
                    )
                    .print()
                    .unwrap();
                    
                std::process::exit(1);
            });
        }

        "repl" => {
            println!("Modu REPL");

            let context = &mut std::collections::HashMap::new();
            
            let mut current_line = 0;
            let mut history: Vec<String> = Vec::new();
            let mut open_function = false;
            let mut input = String::new();

            loop {
                current_line += 1;

                if open_function {
                    print!("|   ");
                } else {
                    input.clear();

                    print!("> ");
                }

                std::io::stdout().flush().unwrap();

                std::io::stdin().read_line(&mut input).unwrap();

                history.push(input.clone());

                if input.contains("{") {
                    open_function = true;
                }

                if input.contains("}") {
                    open_function = false;
                }

                if !open_function {
                    parser::parse(&input, context).unwrap_or_else(|e| {
                        println!("\n⚠️  {}", e.0);
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
                    });
                }
            }
        }

        _ => {
            println!("Invalid action");
        }
    }
}
