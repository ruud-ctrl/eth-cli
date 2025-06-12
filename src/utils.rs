use std::{fs, path::PathBuf};

// use ethers::types::U256;

// pub fn format_ether(balance: U256) -> String {
//     use ethers::utils::format_units;
//     match format_units(balance, "ether") {
//         Ok(s) => s,
//         Err(_) => "<invalid balance>".to_string(),
//     }
// }

pub fn config_file_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push(".eth-cli");
    fs::create_dir_all(&path).ok();
    path.push("config.json");
    path
}