use crate::utils;

use std::{collections::HashMap, fs};
use utils::config_file_path;

const VALID_KEYS: &[&str] = &["rpc_url", "network", "api_key"];

pub fn set_config(key: &str, value: &str) {
    if !VALID_KEYS.contains(&key) {
        eprintln!("Invalid config key: '{}'. Valid keys are: {:?}", key, VALID_KEYS);
        return;
    }

    if key == "rpc_url" && !value.starts_with("http") {
        eprintln!("Invalid RPC URL. It must start with 'http'.");
        return;
    }

    let path = config_file_path();
    let mut config: HashMap<String, String> = if path.exists() {
        let contents = fs::read_to_string(&path).unwrap_or_else(|_| "{}".to_string());
        serde_json::from_str(&contents).unwrap_or_default()
    } else {
        HashMap::new()
    };

    config.insert(key.to_string(), value.to_string());

    if let Err(e) = fs::write(&path, serde_json::to_string_pretty(&config).unwrap()) {
        eprintln!("Failed to write config: {}", e);
    } else {
        println!("Set config '{}': '{}'", key, value);
    }
}

pub fn get_config() {
    let path = config_file_path();
    match fs::read_to_string(&path) {
        Ok(content) => println!("Config:\n{}", content),
        Err(_) => println!("⚠️ No config found."),
    }
}

pub fn reset_config() {
    let path = config_file_path();
    let config: HashMap<String, String> = VALID_KEYS.iter()
        .map(|&k| (k.to_string(), String::new()))
        .collect();

    if let Err(e) = fs::write(&path, serde_json::to_string_pretty(&config).unwrap()) {
        eprintln!("❌ Failed to reset config: {}", e);
    } else {
        println!("🔁 Config reset. Keys initialized: {:?}", VALID_KEYS);
    }
}
