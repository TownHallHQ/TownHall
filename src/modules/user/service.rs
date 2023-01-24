use std::sync::Arc;

use crate::{context::Store, modules::user::model::User};

pub struct UserService {
    store: Arc<Store>,
}

impl UserService {
    pub fn new(store: Arc<Store>) -> Self {
        Self { store }
    }

    pub fn get(&self) {
        let id = String::from("user_1");
        let user = self.store.db.get(id.as_bytes()).unwrap().unwrap();
        let decode: User = bincode::deserialize(&user).unwrap();

        println!("{:?}", decode);
    }

    pub fn create(&self) {
        let id = String::from("user_1");
        let user = User {
            id: 1_i32,
            email: String::from("dave136@gmail.com"),
            name: String::from("Dave"),
            hash: String::from("123456Mm!"),
            last_name: String::from("Arenas"),
            created_at: Default::default(),
            updated_at: Default::default(),
        };

        let encoded = bincode::serialize(&user).unwrap();

        self.store.db.insert(id.as_bytes(), encoded).unwrap();
        println!("User created!");
    }
}
