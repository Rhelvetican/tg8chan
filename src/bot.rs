use crate::config::Config;

pub struct Bot<'a> {
    pub name: &'a str,
    pub id: u64,
    pub token: &'a str,
    pub secret: &'a str,
}

impl<'a> Bot<'a> {
    pub fn from_config(config: &'a Config) -> Self {
        Self {
            name: &config.name,
            id: config.api.id,
            token: &config.api.token,
            secret: &config.api.secret,
        }
    }
}
