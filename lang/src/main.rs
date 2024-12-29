#![feature(internal_output_capture)]

mod lexer;
mod ast;
mod parser;
mod eval;
mod utils;
mod internal;
mod cli;
mod packages;


fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Commands:
    run <file>     - Run a Modu file
    repl           - Start the Modu REPL
    server [port]  - Start the Modu server, default port is 2424
    login          - Login with Modu Packages");
        return;
    }

    let action = &args[1];

    match action.as_str() {
        "run" => cli::run::run(),

        "repl" => cli::repl::repl(),

        "server" => cli::server::server(),

        "login" => cli::login::login(),

        _ => {
            println!("Invalid action");
        }
    }
}