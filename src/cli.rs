use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Cli {
    /// File to read from
    #[arg(short = 'f', long = "file")]
    pub read_path: Option<PathBuf>,

    /// File to write output to
    #[arg(short, long = "write")]
    pub write_path: Option<PathBuf>,

    /// Replace propositions in a file for their simplified form
    #[arg(short, long)]
    pub replace: bool,

    /// Evaluate a proposition for all the possible variable values (default mode)
    #[arg(short, long)]
    pub evaluate: bool,

    /// Just simplify a proposition instead of also evaluating it
    #[arg(short, long)]
    pub simplify: bool
}
