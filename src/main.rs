use anyhow::Result;
use config::load_config;

mod bot;
mod config;

#[tokio::main]
async fn main() -> Result<()> {
    let config = load_config();
    Ok(())
}
