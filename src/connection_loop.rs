use tokio::time::{sleep, Duration};
use url::Url;
use crate::config::{Config};
use futures_util::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use crate::handle_payload::{handle_payload};

pub async fn run_feed() -> Result<(), String> {

let config = Config::get_config()?;

    loop {
        let websocket_str = &config.websocket;

        let ws_url = Url::parse(&websocket_str)
        .map_err(|e| format!("Invalid WebSocket URL format in config:\n{}", e))?;

        match connect_async(ws_url.as_str()).await {
            Ok((ws_stream, _)) => {

                let (_write_pipe, mut read_pipe) = ws_stream.split();

                while let Some(msg) = read_pipe.next().await {
                    match msg {
                        Ok(Message::Text(text)) => {
                            handle_payload(&text)?;
                        },
                        Ok(Message::Close(_)) | Err(_) => {
                            break;
                        },
                        _ => {}
                    }
                }
            },
            Err(e) => {
                eprintln!("Failed WebSocket handhske:\n{}",e);
            }
        }
        println!("Connection Dropped Reconneting in {} Seconds...", config.timeout);
        sleep(Duration::from_secs(config.timeout)).await;
        
    }
}

