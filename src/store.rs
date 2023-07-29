use crate::settings;
use anyhow::Result;
use redis::Commands;

pub struct Store {
    redis_client: redis::Client,
}

impl Store {
    pub fn new() -> Result<Self> {
        let settings = settings::fetch()?;
        let redis_connection_string = settings.get_string("redis_connection_string")?;
        let redis_client = redis::Client::open(redis_connection_string)?;

        Ok(Self { redis_client })
    }

    pub fn get(&self, key: &str) -> Result<String> {
        let mut connection = self.redis_client.get_connection()?;
        let value: String = connection.get(key)?;

        Ok(value)
    }

    pub fn set(&self, key: &str, value: String) -> Result<()> {
        let mut connection = self.redis_client.get_connection()?;
        connection.set(key, value)?;

        Ok(())
    }
}
