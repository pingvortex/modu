use crate::utils;
use crate::parser::parse;
use bat::PrettyPrinter;
use bat::line_range::{LineRanges, LineRange};

use std::io::Write;

pub fn repl() {
    println!("Modu REPL");

    let context = &mut utils::create_context();
    
    let mut current_line = 0;
    let mut history: Vec<String> = Vec::new();
    let mut open_functions = 0;
    let mut input = String::new();

    loop {
        current_line += 1;

        if open_functions > 0 {
            print!("|{}", " ".repeat(open_functions * 4));
        } else {
            input.clear();

            print!("> ");
        }

        std::io::stdout().flush().unwrap();

        let mut this_input = String::new();

        std::io::stdin().read_line(&mut this_input).unwrap();

        history.push(input.clone());

        if this_input.contains("{") {
            open_functions += 1;
        }

        if this_input.contains("}") {
            open_functions -= 1;
        }

        input.push_str(&this_input);

        if open_functions == 0 {
            parse(&input, context).unwrap_or_else(|e| {
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