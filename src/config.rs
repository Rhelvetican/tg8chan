use std::{fs::File, io::Write};

use anyhow::Result;
use dotenvy::{dotenv, var};

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub owner_id: u64,
    pub api: ApiConfig,
    pub file: FileConfig,
    pub policy: PostPolicy,
    pub media: MediaPolicy,
}

impl Config {
    pub fn load() -> Result<Config> {
        dotenv()?;
        Ok(Config {
            owner_id: var("OWNER_ID")?.parse()?,
            api: ApiConfig::load()?,
            file: FileConfig::load()?,
            policy: PostPolicy::load()?,
            media: MediaPolicy::load()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ApiConfig {
    pub id: u64,
    pub secret: String,
    pub token: String,
}

impl ApiConfig {
    pub fn load() -> Result<ApiConfig> {
        dotenv()?;
        Ok(ApiConfig {
            id: var("API_ID")?.parse()?,
            secret: var("API_HASH")?,
            token: var("BOT_TOKEN")?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct PostPolicy {
    pub interval: u64,
    pub restrict_dislike_limit: u64,
    pub delete_dislike_limit: u64,
    pub pin_like_limit: u64,
    pub auto_delete_cnt: u64,
}

impl PostPolicy {
    pub fn load() -> Result<PostPolicy> {
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

#[derive(Debug, Clone, PartialEq)]
pub struct MediaPolicy {
    pub max_media_size: u64,
}

impl MediaPolicy {
    pub fn load() -> Result<MediaPolicy> {
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
