use std::sync::Arc;

use crate::{context::Store, modules::link::model::Link, shared::repository::Repository};

use super::repository::{CreateLinkDto, LinkRepository};

pub struct LinkService {
    repository: Arc<LinkRepository>,
}

impl LinkService {
    pub fn new(store: Arc<Store>) -> Self {
        Self {
            repository: Arc::new(LinkRepository::new(store)),
        }
    }

    #[allow(dead_code)]
    pub fn list(&self) -> Vec<Link> {
        self.repository.list().unwrap()
    }

    #[allow(dead_code)]
    pub fn get(&self, id: String) -> Option<Link> {
        self.repository.find_by_id(id).unwrap()
    }

    pub fn create(&self, dto: CreateLinkDto) -> Link {
        self.repository.create(dto).unwrap()
    }
}
