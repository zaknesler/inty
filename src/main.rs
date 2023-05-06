#![allow(dead_code)]

mod args;
mod ast;
mod eval;
mod lexer;
mod parser;
mod token;

use clap::Parser;
use std::io::{self, BufRead, Write};

fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();

    match args.command {
        args::Command::Exec { file } => {
            let input = std::fs::read_to_string(file)?;
            let value = process_string(input, args.debug)?;

            println!("{}", value);
        }

        args::Command::Repl => loop {
            print!("> ");
            io::stdout().flush().expect("Could not flush output");

            let mut line = String::new();
            let stream = io::stdin();
            let bytes = stream
                .lock()
                .read_line(&mut line)
                .expect("Could not read line");

            if bytes == 0 {
                break;
            }

            if line.trim().is_empty() {
                continue;
            }

            match process_string(line, args.debug) {
                Ok(value) => println!("{}", value),
                Err(err) => println!("{}", err),
            }
        },
    }

    Ok(())
}

fn process_string(input: String, debug: bool) -> anyhow::Result<i32> {
    // Tokenize input
    let lexer = lexer::Lexer::new(input);
    let tokens = lexer.tokenize()?;

    // Parse tokens into AST
    let mut parser = parser::Parser::new(&tokens);
    let ast = parser.parse()?;

    if debug {
        dbg!(&ast);
    }

    // Evaluate the AST
    let evaluator = eval::Evaluator::new(&ast);
    Ok(evaluator.eval())
}
