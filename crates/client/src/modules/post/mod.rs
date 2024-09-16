pub mod post_create;
pub mod posts;

use anyhow::Result;
use pxid::Pxid;
use reqwest::{Client, Url};

use crate::post::post_create::post_create::PostCreateInput;

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

    pub async fn post_create(&self, input: PostCreateInput) -> Result<post_create::PostCreate> {
        post_create::posts(self, input).await
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
