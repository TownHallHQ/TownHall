mod modules;

pub use modules::*;

pub(crate) const GRAPHQL_PATH: &str = "/graphql";

use std::fmt::Display;

use anyhow::{anyhow, Result};
use reqwest::Url;

use auth::AuthClient;

pub struct Client {
    pub auth: AuthClient,
}

impl Client {
    pub fn new<T: Clone + Display + TryInto<Url>>(domain: T) -> Result<Self> {
        let domain = domain
            .clone()
            .try_into()
            .map_err(|_| anyhow!("Provided domain \"{domain}\" is not a valid Url."))?;

        Ok(Self {
            auth: AuthClient::new(domain.clone()),
        })
    }
}
