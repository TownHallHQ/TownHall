use crate::config::Config;

#[derive(Clone)]
pub struct Context;

impl Context {
    pub async fn new(config: Config) -> Self {
        Self
    }
}
