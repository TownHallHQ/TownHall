use std::env;

pub struct Config {
    pub cors_allow_origin: String,
    pub database_url: String,
    pub server_port: u16,
}

impl Config {
    pub fn new() -> Self {
        let cors_allow_origin = env::var("CORS_ALLOW_ORIGIN").unwrap();
        let database_url = env::var("DATABASE_URL").unwrap();
        let server_port = env::var("PORT").unwrap().parse::<u16>().unwrap();

        Self {
            cors_allow_origin,
            database_url,
            server_port,
        }
    }
}
