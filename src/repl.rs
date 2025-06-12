use ethers::providers::{Http, Provider, Middleware};
use std::env;
use std::str::FromStr;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use rustyline::history::DefaultHistory;
use ethers::types::Address;
use crate::utils::format_ether;

fn get_rpc_url() -> Result<String, String> {
    env::var("ETH_CLI_RPC").map_err(|_| "ETH_CLI_RPC environment variable is not set".to_string())
}

pub fn set_rpc_url(url: &str) {
    let export_line = format!("export ETH_CLI_RPC=\"{}\"", url);
    println!("✅ Set RPC URL. To persist it, add this to your shell profile (~/.bashrc, ~/.zshrc):\n{}",
        export_line);
}

pub async fn handle_line(line: &str, provider: &Provider<Http>) -> Option<String> {
    if let Ok(addr) = Address::from_str(line) {
        match provider.get_balance(addr, None).await {
            Ok(balance) => Some(format!("Balance: {} ETH", format_ether(balance))),
            Err(_) => Some("Error fetching balance".to_string()),
        }
    } else {
        Some("Unrecognized input".to_string())
    }
}
