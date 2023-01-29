use std::sync::Arc;

use crate::{context::Store, modules::link::model::Link};

pub struct LinkService {
    store: Arc<Store>,
}

pub struct CreateLinkDto {
    pub original_url: String,
    pub owner_id: Option<String>,
}

impl From<CreateLinkDto> for Link {
    fn from(value: CreateLinkDto) -> Self {
        Self {
            id: String::from(""),
            hash: String::from(""),
            original_url: value.original_url,
            owner_id: value.owner_id,
            created_at: Default::default(),
            expires_at: Default::default(),
            updated_at: Default::default(),
        }
    }
}

impl LinkService {
    pub fn new(store: Arc<Store>) -> Self {
        Self { store }
    }

    pub fn get_all(&self) -> Vec<Link> {
        let link_tree = self.store.db.open_tree("links").unwrap();
        link_tree
            .iter()
            .map(|item| {
                let (_, v) = item.unwrap();
                bincode::deserialize(&v).unwrap()
            })
            .collect()
    }

    pub fn get(&self, id: String) -> Link {
        let link_tree = self.store.db.open_tree("links").unwrap();
        let link = link_tree.get(id).unwrap().unwrap();

        bincode::deserialize(&link).unwrap()
    }

    pub fn create(&self, link: CreateLinkDto) -> String {
        let link_tree = self.store.db.open_tree("links").unwrap();
        let id = self.store.generate_id();
        let mut link = Link::from(link);
        link.id = id;
        let encoded = bincode::serialize(&link).unwrap();

        link_tree.insert(&link.id, encoded).unwrap();
        println!("Creating new link");
        link.id
    }
}
