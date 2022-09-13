use std::env;

pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn new() -> Self {
        let database_url = env::var("DATABASE_URL").unwrap();

        Self { database_url }
    }
}
