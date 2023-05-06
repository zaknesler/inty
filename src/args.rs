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
    /// Execute source code from a file
    Exec {
        /// File containing source code
        #[clap(name = "FILE")]
        file: String,
    },

    /// Start interactive session
    Repl,
}
