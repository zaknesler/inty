#![allow(dead_code)]

mod args;
mod core;
mod eval;
mod lexer;
mod parser;

use crate::core::*;
use clap::Parser;
use std::io::{self, BufRead, Write};

fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();

    match args.command {
        args::Command::Run { file } => {
            let input = std::fs::read_to_string(file)?;
            let value = process_string(input, args.debug)?;
            println!("{:?}", value);
        }

        args::Command::Eval { expr } => {
            let value = process_string(expr, args.debug)?;
            println!("{:?}", value);
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
                Ok(value) => println!("{:?}", value),
                Err(err) => println!("{}", err),
            }
        },
    }

    Ok(())
}

pub fn process_string(input: String, debug: bool) -> anyhow::Result<ProgramOutput> {
    // Tokenize input
    let lexer = lexer::Lexer::new(input);
    let tokens = lexer.tokenize()?;

    if debug {
        dbg!(&tokens);
    }

    // Parse tokens
    let mut parser = parser::Parser::new(&tokens);
    let stmts = parser.parse()?;

    if debug {
        dbg!(&stmts);
    }

    // Evaluate the parsed statements
    let mut evaluator = eval::Evaluator::new(stmts);
    evaluator.eval()
}

#[cfg(test)]
mod tests {
    use crate::process_string;

    #[test]
    fn expression_evaluation() {
        let start = line!() + 2;
        [
            ("1", 1),
            ("1 + 2 + 3", 6),
            ("1 + 2", 3),
            ("5 - 2", 3),
            ("-1", -1),
            ("-5 - 2", -7),
            ("4 + 4 + 4", 12),
            ("6 - 2 + 5", 9),
            ("10 - 2 + 3", 11),
            ("-2 + (-5)", -7),
            ("2 * 3 + 4", 10),
            ("2 + 3 * 4", 14),
            ("2 + 3 * 5", 17),
            ("(2 + 3) * (4 - 1)", 15),
            ("(2 + 3) * 4", 20),
            ("(4 + 2) * 3", 18),
            ("(9 + 1) * (5 - 2)", 30),
            ("1 * 2 * 3 * 4", 24),
            ("1 * 2", 2),
            ("3 * 3 * 3", 27),
            ("4 * 5 - 3", 17),
            ("7 * 2", 14),
            ("10 / 2", 5),
            ("10 / 5", 2),
            ("10 * 10 / 5", 20),
            ("1 * 4 * 5 / 10 / 2 * 10", 10),
            ("3 ^ 0", 1),
            ("3 ^ 1", 3),
            ("3 ^ 2", 9),
            ("3 ^ 3", 27),
            ("-3 ^ 2", -9),
            ("-3 ^ 3", -27),
            ("(-3) ^ 2", 9),
            ("(-3) ^ 3", -27),
            ("2 ^ 3 + 4", 12),
            ("3 ^ 2 + 4", 13),
            ("(-3) ^ 2 + 4", 13),
            ("2 ^ 3 * 4 + 4", 36),
            ("3 ^ 2 * 4 + 4", 40),
            ("(-3) ^ 2 * 4 + 4", 40),
        ]
        .into_iter()
        .enumerate()
        .for_each(|(index, (string, val))| {
            let results = process_string(string.to_string(), false).unwrap();

            assert_eq!(
                results[0],
                val,
                "expression = \"{}\" (line {})",
                string,
                start + (index as u32)
            );
        })
    }
}
