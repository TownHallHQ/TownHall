use std::path::Path;

use rand::{distributions::Alphanumeric, Rng};
use sled::{Config, Db};

pub struct Store {
    pub db: Db,
}

impl Store {
    pub fn new<T: AsRef<Path>>(path: T) -> Self {
        let sled_config = Config::new()
            .path(path)
            .temporary(false)
            .use_compression(false);

        let db = sled_config.open().expect("Failed to create Sled instance");

        Self { db }
    }

    pub fn generate_id(&self) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect::<String>()
    }
}
