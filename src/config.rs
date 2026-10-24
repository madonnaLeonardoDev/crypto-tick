use std::{env, fs};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SymbolWs{
    pub symbol: String,
    pub endpoints: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub websocket: String,
    pub watchlist: Vec<SymbolWs>,
    pub execution_endpoint: String,
    pub timeout: u64,
    pub logging: bool,
    pub logs_path: String
}

impl Config {
    fn config_path() -> Result<PathBuf, String> {
        let home_path = env::var("HOME")
        .map_err(|_| "HOME enviroment variable not set".to_string())?;

        let mut config_dir = PathBuf::from(home_path);
        config_dir.push(".config");

        // Ensure ~/.config/ exists before trying to write inside it
        fs::create_dir_all(&config_dir)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;

        config_dir.push("crypto-tick.conf");

        if !Path::new(&config_dir).exists() {
            let default_config = 
r#"#crypto-tick config file
            
#API endpoint for market data websocket
websocket = ""

#Watchlist
#WARNING: The source code is written to parse Binance's aggTrade, bookTicker, depth<depthLevels>@<Frequency>
#If using different API you might wanna update the parsing source code (handle_payload.rs)

[[watchlist]]
symbol = ""
endpoints = [""]

#API endpoint for execution 
execution_endpoint = ""

#Retry timeout after connection dropped (in seconds)
timeout = 3

logging = false

#logs_path if left empty defaults to your home directory
logs_path = ""
            "#;

            fs::write(&config_dir, &default_config)
                .map_err(|e| e.to_string())?;

            return Ok(config_dir)
        };

        return Ok(config_dir)

    }


    pub fn get_config() -> Result<Self, String> {

        let config_dir = Config::config_path()?;

        let config_str = fs::read_to_string(config_dir)
            .map_err(|e| e.to_string())?;

        let mut parsed_config: Config = toml::from_str(&config_str)
            .map_err(|e| e.to_string())?;

        if parsed_config.execution_endpoint.is_empty() || parsed_config.websocket.is_empty() || parsed_config.watchlist.is_empty() {
            return Err("Missing fields in the config file".to_string())
        }

        if parsed_config.logs_path.is_empty() {
            parsed_config.logs_path = env::var("HOME")
                .map_err(|_| "Missing Home enviroment variable".to_string())?;
        }

        Ok(parsed_config)
    }
}