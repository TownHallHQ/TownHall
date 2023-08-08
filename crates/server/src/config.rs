use std::env;

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub server_port: u16,
    // TODO: Find a way to support other implementors for ImageProvider
    pub minio_username: String,
    pub minio_password: String,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        let database_url = env::var("DATABASE_URL").unwrap();
        let jwt_secret = env::var("JWT_SECRET").unwrap();
        let server_port = env::var("PORT").unwrap().parse::<u16>().unwrap();
        let minio_username = env::var("MINIO_ROOT_USER").unwrap();
        let minio_password = env::var("MINIO_ROOT_PASSWORD").unwrap();

        Self {
            database_url,
            jwt_secret,
            server_port,
            minio_username,
            minio_password,
        }
    }
}
