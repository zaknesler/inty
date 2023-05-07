use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "inty", version, author, about, long_about = None)]
pub struct Args {
    /// Enable debug mode
    #[clap(long, short, action)]
    pub debug: bool,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Parser)]
pub enum Command {
    /// Run source code from a file
    Run {
        /// File containing source code
        #[clap(name = "FILE")]
        file: String,
    },

    /// Evaluate inline expression
    Eval {
        /// Inline expression (e.g. "2 + 3")
        #[clap(name = "EXPRESSION")]
        expr: String,
    },

    /// Start interactive session
    Repl,
}
