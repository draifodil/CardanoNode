// src/main.rs
/*
 * Main executable for CardanoNode
 */

use clap::Parser;
use cardanonode::{Result, run};

#[derive(Parser)]
#[command(version, about = "CardanoNode - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Input file path
    #[arg(short, long)]
    input: Option<String>,
    
    /// Output file path
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose, args.input, args.output)
}
