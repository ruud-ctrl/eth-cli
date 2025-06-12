use std::collections::HashMap;
use std::fs;
use ethers::providers::{Http, Provider};
use utils::config_file_path;

use crate::utils;

pub fn get_provider() -> Result<Provider<Http>, String> {
    let config_path = config_file_path();
    let contents = fs::read_to_string(&config_path).map_err(|_| "Could not read config file")?;
    let config: HashMap<String, String> = serde_json::from_str(&contents).unwrap_or_default();

    let binding = "".to_string();
    let base_url = config.get("rpc_url").unwrap_or(&binding);
    let api_key = config.get("api_key").unwrap_or(&binding);
    let full_url = format!("{}{}", base_url, api_key);

    if full_url.len() < 10 {
        return Err("RPC URL is not properly configured".to_string());
    }

    Provider::<Http>::try_from(full_url.as_str()).map_err(|e| e.to_string())
}
