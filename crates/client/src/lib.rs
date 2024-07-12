mod modules;

pub use modules::*;

use auth::AuthClient;

pub struct Client {
    pub auth: AuthClient,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    pub fn new() -> Self {
        Self {
            auth: AuthClient::new(),
        }
    }
}
