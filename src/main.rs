mod args;
mod ast;
mod lexer;
mod parser;
mod token;

use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();

    // Read file into string
    match args.command {
        args::Command::Run { file } => {
            let input = std::fs::read_to_string(file).unwrap();

            // Tokenize
            let lexer = lexer::Lexer::new(input);
            let tokens = lexer.tokenize()?;

            let mut parser = parser::Parser::new(tokens);
            let ast = parser.parse()?;

            dbg!(ast);
        }
    }

    Ok(())
}
