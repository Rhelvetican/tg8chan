use config::{init_config, Config};

mod cmd;
mod config;
mod db;
mod error;

fn main() {
    let config = match Config::load() {
        Ok(config) => config,
        Err(_) => {
            eprintln!("Unable to read from .env file.");
            println!("Automatically generating a new .env file.");
            init_config().unwrap();
            Config::load().unwrap()
        }
    };
}
