use std::sync::Arc;

use serde::Serialize;

use crate::context::Store;
use crate::shared::repository::Repository;

use super::model::Link;

const LINK_REPOSITORY_TREE: char = 'l';

pub struct LinkRepository {
    store: Arc<Store>,
}

impl LinkRepository {
    pub fn new(store: Arc<Store>) -> Self {
        Self { store }
    }
}

impl Repository<LINK_REPOSITORY_TREE, Link> for LinkRepository {
    type Error = ();

    fn get_tree(&self) -> sled::Result<sled::Tree> {
        self.store.db.open_tree(LINK_REPOSITORY_TREE.to_string())
    }
}

#[derive(Serialize)]
pub struct CreateLinkDto {
    pub original_url: String,
    pub owner_id: Option<String>,
    pub custom_hash: Option<String>,
}

impl From<CreateLinkDto> for Link {
    fn from(value: CreateLinkDto) -> Self {
        Self {
            id: String::from(""),
            hash: String::from(""),
            original_url: value.original_url,
            owner_id: value.owner_id,
            ..Default::default()
        }
    }
}
