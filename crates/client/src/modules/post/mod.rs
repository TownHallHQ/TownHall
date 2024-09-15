pub mod posts;

use anyhow::Result;
use pxid::Pxid;
use reqwest::{Client, Url};

pub struct PostClient {
    client: Client,
    domain: Url,
}

impl PostClient {
    pub fn new(domain: Url) -> Self {
        Self {
            domain,
            client: Client::new(),
        }
    }

    pub async fn posts(
        &self,
        after: Option<Pxid>,
        before: Option<Pxid>,
        first: Option<i64>,
        last: Option<i64>,
    ) -> Result<posts::Posts> {
        posts::posts(self, after, before, first, last).await
    }
}
