use crate::settings;
use anyhow::{Context, Result};
use config::Config;
use redis::Commands;

pub struct Store {
    redis_client: redis::Client,
    settings: Config,
}

impl Store {
    pub fn new() -> Result<Self> {
        let settings = settings::fetch()?;

        let redis_connection_string = settings
            .get_string("redis_connection_string")
            .context("Redis connection string not set, please run `dropzone init`")?;

        let redis_client = redis::Client::open(redis_connection_string)
            .context("Failed to setup Redis connection")?;

        Ok(Self {
            redis_client,
            settings,
        })
    }

    pub fn get(&self, key: &str) -> Result<String> {
        let mut connection = self
            .redis_client
            .get_connection()
            .context("Failed to connect to Redis")?;
        let value: String = connection.get(key).context("Key missing / Invalid value")?;

        Ok(value)
    }

    pub fn set(&self, key: &str, value: String) -> Result<()> {
        let mut connection = self
            .redis_client
            .get_connection()
            .context("Failed to connect to Redis")?;
        connection.set(key, value).context("Failed to set value")?;

        if let Ok(expiry) = self.settings.get_int("expiry") {
            connection
                .expire(key, expiry as usize)
                .context("Failed to set expiry")?;
        }

        Ok(())
    }
}
