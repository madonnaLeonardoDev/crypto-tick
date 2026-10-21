use crate::config::{Config};
use url::Url;

pub fn build_url(config: &Config) -> Result<Url, String> {
    let websocket_url = &config.websocket;

    let mut endpoints: Vec<String> = Vec::new();

    for s in &config.watchlist {
        for endpoint_str in &s.endpoints {
            endpoints.push(format!("{}@{}", s.symbol.to_ascii_lowercase(), endpoint_str));
        }
    }



    Ok(Url::parse(&format!("{}{}",websocket_url, endpoints.join("/"))) 
        .map_err(|_| "Invalid Url endpoint".to_string())?)
}