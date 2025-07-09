// src/main.rs
/*
 * Main executable for ErcNftMetadataIndexerProtocolUltra
 */

use clap::Parser;
use ercnftmetadataindexerprotocolultra::{Result, run};

#[derive(Parser)]
#[command(version, about = "ErcNftMetadataIndexerProtocolUltra - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
