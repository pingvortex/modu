#![feature(internal_output_capture)]

mod lexer;
mod ast;
mod parser;
mod eval;
mod utils;
mod internal;
mod cli;


fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Commands:
    run <file>     - Run a Modu file
    repl           - Start the Modu REPL
    server [port]  - Start the Modu server, default port is 2424");
        return;
    }

    let action = &args[1];

    match action.as_str() {
        "run" => cli::run::run(),

        "repl" => cli::repl::repl(),

        "server" => cli::server::server(),

        _ => {
            println!("Invalid action");
        }
    }
}