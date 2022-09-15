use std::env;

pub struct Config {
    pub database_url: String,
    pub server_port: u16,
}

impl Config {
    pub fn new() -> Self {
        let database_url = env::var("DATABASE_URL").unwrap();
        let server_port = env::var("PORT").unwrap().parse::<u16>().unwrap();

        Self {
            database_url,
            server_port,
        }
    }
}
