use std::path::Path;

use sled::{Config, Db};

pub struct Store {
    db: Db,
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
}
