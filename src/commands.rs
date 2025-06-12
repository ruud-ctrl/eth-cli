use clap::{Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    Greet {
        #[arg(short, long)]
        name: String,
    },

    SetConfig {
        #[arg(short, long)]
        key: String,
    
        #[arg(short, long)]
        value: String,
    },
    
    GetConfig {},
    ResetConfig {},

    GetBalance {
        #[arg(short, long)]
        address: String,
    },

    GetTransactionCount {
        #[arg(short, long)]
        address: String,
    }, 

    GetBlockNumber {

    },

    GetChainId {

    },

    GetGasPrice {

    }



}


    // if let Some(addr_str) = cli.address {
    //     let provider = match Provider::<Http>::try_from(cli.rpc_url.as_str()) {
    //         Ok(p) => p,
    //         Err(e) => {
    //             eprintln!("Failed to create provider: {}", e);
    //             return;
    //         }
    //     };

    //     let address = match Address::from_str(&addr_str) {
    //         Ok(addr) => addr,
    //         Err(_) => {
    //             eprintln!("Invalid Ethereum address");
    //             return;
    //         }
    //     };

    //     match provider.get_balance(address, None).await {
    //         Ok(balance) => println!("Balance: {} ETH", format_ether(balance)),
    //         Err(e) => eprintln!("Failed to fetch balance: {}", e),
    //     }
    // } else {
    //     println!("No address provided. Use --help for options.");
    // }