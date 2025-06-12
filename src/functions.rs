use crate::rpc::get_provider;
use ethers::{providers::Middleware, types::Address};
use std::str::FromStr;

pub fn greet(name: &str) {
    if name.is_empty() {
        eprintln!("Name cannot be empty!");
    } else {
        println!("👋 Hello, {}!", name);
    }
}


pub async fn get_balance(address: &str) {
    if !address.starts_with("0x") || address.len() != 42 {
        eprintln!("Invalid Ethereum address");
        return;
    }

    let addr = match Address::from_str(address) {
        Ok(a) => a,
        Err(_) => {
            eprintln!("Failed to parse address");
            return;
        }
    };

    let provider = match get_provider() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Provider error: {}", e);
            return;
        }
    };

    match provider.get_balance(addr, None).await {
        Ok(balance) => println!("Balance: {} wei", balance),
        Err(e) => eprintln!("Failed to get balance: {}", e),
    }
}

pub async fn get_transaction_count(address: &str) {
    let addr = match Address::from_str(address) {
        Ok(a) => a,
        Err(_) => {
            eprintln!("Failed to parse address");
            return;
        }
    };

    let provider = match get_provider() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Provider error: {}", e);
            return;
        }
    };

    match provider.get_transaction_count(addr, None).await {
        Ok(nonce) => println!(" Nonce: {}", nonce),
        Err(e) => eprintln!("Failed to get nonce: {}", e),
    }
}

pub async fn get_block_number() {
    let provider = match get_provider() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Provider error: {}", e);
            return;
        }
    };

    match provider.get_block_number().await {
        Ok(result) => println!(" BlockNumber: {}", result),
        Err(e) => eprintln!("Failed to get BlockNumber: {}", e),
    }
}

pub async fn get_chain_id() {
    let provider = match get_provider() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Provider error: {}", e);
            return;
        }
    };

    match provider.get_chainid().await {
        Ok(result) => println!(" ChainID: {}", result),
        Err(e) => eprintln!("Failed to get ChainID: {}", e),
    }
}

pub async fn get_gas_price() {
    let provider = match get_provider() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Provider error: {}", e);
            return;
        }
    };

    match provider.get_gas_price().await {
        Ok(result) => println!(" GasPrice: {}", result),
        Err(e) => eprintln!("Failed to get GasPrice: {}", e),
    }
}
