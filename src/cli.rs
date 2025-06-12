use crate::commands::Commands;
use clap::Parser;

#[derive(Parser)]
#[command(name = "eth-cli")]
#[command(about = "A CLI tool to interact with Ethereum", long_about = "A CLI tool to interact with Ethereum, etc.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
