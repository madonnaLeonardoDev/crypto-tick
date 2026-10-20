mod config;
mod connection_loop;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing crypto-tick engine...");

    // Call and await your async feed loop
    if let Err(e) = connection_loop::run_feed().await {
        eprintln!("Critical feed failure: {}", e);
    }

    Ok(())
}
