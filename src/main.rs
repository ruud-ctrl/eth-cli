mod cli;
mod commands;
mod utils;
mod functions;
mod config;
mod rpc;

use commands::Commands;
use clap::Parser;
use cli::Cli;
use functions::{greet};
use config::{set_config, get_config, reset_config};

use crate::functions::{get_balance, get_block_number, get_chain_id, get_gas_price, get_transaction_count};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Greet { name } => greet(&name),
        Commands::GetConfig {} => get_config(),
        Commands::ResetConfig {} => reset_config(),
        Commands::SetConfig {key, value } => set_config(&key, &value),
        Commands::GetBalance {address } => get_balance(&address).await,
        Commands::GetTransactionCount { address } => get_transaction_count(&address).await,
        Commands::GetBlockNumber { } => get_block_number().await,
        Commands::GetGasPrice { } => get_gas_price().await,
        Commands::GetChainId { } => get_chain_id().await
    }
}