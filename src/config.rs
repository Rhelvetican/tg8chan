use anyhow::Result;
use dotenvy::{dotenv, var};
use jsonutils::file::{read_json, write_json};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, path::Path};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Config {
    pub owner_id: u64,
    pub name: String,
    pub api: ApiConfig,
    pub file: FileConfig,
    pub policy: PostPolicy,
    pub media: MediaPolicy,
}

impl Config {
    pub fn load_from_dotenv() -> Result<Self> {
        dotenv()?;
        Ok(Config {
            owner_id: var("OWNER_ID")?.parse()?,
            name: var("NAME")?,
            api: ApiConfig::load()?,
            file: FileConfig::load()?,
            policy: PostPolicy::load()?,
            media: MediaPolicy::load()?,
        })
    }
    pub fn save_to_json<T: AsRef<Path>>(&self, path: T) -> Result<()> {
        write_json(path, self)
    }
    pub fn load_from_json<T: AsRef<Path>>(path: T) -> Result<Self> {
        read_json(path)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiConfig {
    pub id: u64,
    pub secret: String,
    pub token: String,
}

impl ApiConfig {
    fn load() -> Result<ApiConfig> {
        dotenv()?;
        Ok(ApiConfig {
            id: var("API_ID")?.parse()?,
            secret: var("API_HASH")?,
            token: var("BOT_TOKEN")?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FileConfig {
    pub seed: u64,
    pub db: String,
    pub log: String,
    pub media: String,
}

impl FileConfig {
    pub fn load() -> Result<FileConfig> {
        dotenv()?;
        Ok(FileConfig {
            seed: var("SEED")?.parse()?,
            db: var("DATABASE_FILE")?,
            log: var("LOG_FILE")?,
            media: var("MEDIA_DIR")?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PostPolicy {
    pub interval: u64,
    pub restrict_dislike_limit: u64,
    pub delete_dislike_limit: u64,
    pub pin_like_limit: u64,
    pub auto_delete_cnt: u64,
}

impl PostPolicy {
    fn load() -> Result<PostPolicy> {
        dotenv()?;
        Ok(PostPolicy {
            interval: var("POST_INTERVAL")?.parse()?,
            restrict_dislike_limit: var("RESTRICT_DISLIKE_LIMIT")?.parse()?,
            delete_dislike_limit: var("DELETE_DISLIKE_LIMIT")?.parse()?,
            pin_like_limit: var("PIN_LIKE_LIMIT")?.parse()?,
            auto_delete_cnt: var("AUTODELETE_COUNT")?.parse()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MediaPolicy {
    pub max_media_size: u64,
}

impl MediaPolicy {
    fn load() -> Result<MediaPolicy> {
        dotenv()?;
        Ok(MediaPolicy {
            max_media_size: var("MAX_MEDIA_SIZE")?.parse()?,
        })
    }
}

const DEFAULT_CONFIG: &str = r#"
# Session Name
NAME = "TG8CHAN"

# Telegram API (https://my.telegram.org/)
API_ID = 0
API_HASH = "abcd"
BOT_TOKEN = "wxyz"

# DB and Log
SEED = 10
OWNER_ID = 123
DATABASE_FILE = "database.db"
LOG_FILE = "stats.log"
POST_ID = -456
MEDIA_DIR = "media"

# Posting Policies
POST_INTERVAL = 300
RESTRICT_DISLIKE_LIMIT = 10
DELETE_DISLIKE_LIMIT = 5
PIN_LIKE_LIMIT = 10
AUTODELETE_COUNT = 50

# Media Policies
MAX_MEDIA_SIZE = 2000000
"#;

pub fn init_config() -> Result<()> {
    let mut config = File::create(".env")?;
    config.write_all(DEFAULT_CONFIG.as_bytes())?;
    Ok(())
}

pub fn load_config() -> Config {
    match Config::load_from_dotenv() {
        Ok(config) => config,
        Err(_) => {
            eprintln!("Failed to load config from .env file");
            eprintln!("Trying to read from config/config.json file");
            match Config::load_from_json("config/config.json") {
                Ok(config) => config,
                Err(_) => {
                    eprintln!("Failed to load config from config/config.json file");
                    eprintln!("Trying to create a new config file");
                    match init_config() {
                        Ok(_) => Config::load_from_dotenv().unwrap(),
                        Err(e) => {
                            eprintln!("Failed to create a new config file: {}", e);
                            panic!("Failed to load config")
                        }
                    }
                }
            }
        }
    }
}
