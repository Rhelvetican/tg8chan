use anyhow::Result;

mod bot;
mod config;
mod db;
mod error;
mod log;

#[tokio::main]
async fn main() -> Result<()> {
    Ok(())
}
