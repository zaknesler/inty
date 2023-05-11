#![allow(dead_code)]

mod args;
mod core;
mod eval;
mod lexer;
mod parser;

use crate::core::*;
use clap::Parser as _;
use eval::Evaluator;
use lexer::Lexer;
use parser::Parser;
use rustyline::{error::ReadlineError, DefaultEditor};

fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();

    match args.command {
        args::Command::Run { file } => {
            let mut eval = Evaluator::new();
            let input = std::fs::read_to_string(file)?;
            let values = process_string(&mut eval, input, args.debug)?;
            print_output(&values);
        }

        args::Command::Eval { expr } => {
            let mut eval = Evaluator::new();
            let values = process_string(&mut eval, expr, args.debug)?;
            print_output(&values);
        }

        args::Command::Repl => {
            let mut rl = DefaultEditor::new()?;
            let mut eval = Evaluator::new();

            loop {
                match rl.readline("> ") {
                    Ok(line) => {
                        rl.add_history_entry(line.as_str())?;

                        match process_string(&mut eval, line, args.debug) {
                            Ok(values) => print_output(&values),
                            Err(err) => println!("{}", err),
                        }
                    }
                    Err(ReadlineError::Eof | ReadlineError::Interrupted) => {
                        println!("inty session ended");
                        break;
                    }
                    Err(err) => {
                        println!("Error: {:?}", err);
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}

fn process_string(
    eval: &mut Evaluator,
    input: String,
    debug: bool,
) -> anyhow::Result<Vec<Option<Value>>> {
    // Tokenize input
    let tokens = Lexer::tokenize(input)?;

    if debug {
        dbg!(&tokens);
    }

    // Parse tokens into a list of statements
    let stmts = Parser::new(&tokens).parse()?;

    if debug {
        dbg!(&stmts);
    }

    // Evaluate the parsed statements
    eval.eval(stmts)
}

fn print_output(values: &Vec<Option<Value>>) {
    values.iter().for_each(|v| {
        if let Some(v) = v {
            println!("{}", v);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

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
            let mut eval = Evaluator::new();
            let results = process_string(&mut eval, string.to_string(), false).unwrap();

            assert_eq!(
                results[0],
                Some(Value::Integer(val)),
                "expression = \"{}\" (line {})",
                string,
                start + (index as u32)
            );
        })
    }

    #[test]
    fn let_statement() {
        let mut eval = Evaluator::new();
        let values = process_string(&mut eval, "let x = 42; x".into(), false).unwrap();
        assert_eq!(vec![None, Some(Value::Integer(42))], values);
    }
}
