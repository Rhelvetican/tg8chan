use anyhow::Result;
use config::load_config;

mod cmd;
mod config;
mod db;
mod error;

fn main() -> Result<()> {
    let config = load_config();
    config.save_to_json("config.json")?;
    Ok(())
}
