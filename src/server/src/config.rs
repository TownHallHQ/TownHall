use std::env;
use std::str::FromStr;

use sea_orm::ConnectOptions;

#[derive(Debug)]
pub struct Config {
    pub cors_allow_origin: String,
    pub dbconnect_options: ConnectOptions,
    pub jwt_secret: String,
    pub server_port: u16,
}

impl Config {
    pub fn new() -> Self {
        let cors_allow_origin = Config::env_var::<String>("CORS_ALLOW_ORIGIN");
        let jwt_secret = Config::env_var::<String>("JWT_SECRET");
        let server_port = Config::env_var::<u16>("PORT");
        let database_url = Config::env_var::<String>("DATABASE_URL");
        let dbconnect_options = ConnectOptions::new(database_url);

        Self {
            cors_allow_origin,
            dbconnect_options,
            jwt_secret,
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

    #[allow(dead_code)]
    fn env_var_opt<T: FromStr>(key: &str) -> Option<T> {
        let value = env::var(key).ok()?;

        str::parse::<T>(&value).ok()
    }
}
