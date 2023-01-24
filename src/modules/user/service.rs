use std::sync::Arc;

use crate::context::Store;

pub struct UserService {
    store: Arc<Store>,
}

impl UserService {
    pub fn new(store: Arc<Store>) -> Self {
        Self { store }
    }

    pub fn create(&self) {
        println!("Creating user");
    }
}
