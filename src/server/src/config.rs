use std::env;
use std::str::FromStr;

use sea_orm::ConnectOptions;

#[derive(Debug)]
pub struct Config {
    pub cors_allow_origin: String,
    pub dbconnect_options: ConnectOptions,
    pub jwt_secret: String,
    pub server_port: u16,
    pub use_sqlite: bool,
}

impl Config {
    pub fn new() -> Self {
        let cors_allow_origin = Config::env_var::<String>("CORS_ALLOW_ORIGIN");
        let jwt_secret = Config::env_var::<String>("JWT_SECRET");
        let server_port = Config::env_var::<u16>("PORT");
        let use_sqlite = Config::env_var_opt::<bool>("USE_SQLITE").unwrap_or_default();
        let dbconnect_options = Self::make_connection_opts(use_sqlite);

        Self {
            cors_allow_origin,
            dbconnect_options,
            jwt_secret,
            server_port,
            use_sqlite,
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

    fn env_var_opt<T: FromStr>(key: &str) -> Option<T> {
        let value = env::var(key).ok()?;

        str::parse::<T>(&value).ok()
    }

    fn make_connection_opts(use_sqlite: bool) -> ConnectOptions {
        if use_sqlite {
            return ConnectOptions::new("sqlite://linx_database.db?mode=rwc".into());
        }

        let database_url = Config::env_var::<String>("DATABASE_URL");

        ConnectOptions::new(database_url)
    }
}
