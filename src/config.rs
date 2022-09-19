use std::env;
use std::str::FromStr;

pub struct Config {
    pub cors_allow_origin: String,
    pub database_url: String,
    pub server_port: u16,
}

impl Config {
    pub fn new() -> Self {
        let cors_allow_origin = Config::env_var::<String>("CORS_ALLOW_ORIGIN");
        let database_url = Config::env_var::<String>("DATABASE_URL");
        let server_port = Config::env_var::<u16>("PORT");

        Self {
            cors_allow_origin,
            database_url,
            server_port,
        }
    }

    fn env_var<T: FromStr>(key: &str) -> T {
        let value =
            env::var(key).unwrap_or_else(|_| panic!("Missing environment variable: {}", key));

        if let Ok(parsed) = str::parse::<T>(&value) {
            return parsed;
        }

        panic!(
            "Failed to parse environment variable value from key: {}",
            key
        );
    }
}
