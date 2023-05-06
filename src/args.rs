use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "inty", version, author, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Parser)]
pub enum Command {
    /// Run a file containing inty code
    Run {
        /// File containing source code
        #[clap(name = "FILE")]
        file: String,
    },
}
