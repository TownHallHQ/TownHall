use std::sync::Arc;

use crate::{context::Store, modules::user::model::User};

pub struct UserService {
    store: Arc<Store>,
}

pub struct CreateUserDto {
    pub name: String,
    pub last_name: String,
    pub email: String,
}

impl From<CreateUserDto> for User {
    fn from(value: CreateUserDto) -> User {
        User {
            id: String::from(""),
            name: value.name,
            last_name: value.last_name,
            email: value.email,
            hash: String::from(""),
            created_at: Default::default(),
            updated_at: Default::default(),
        }
    }
}

impl UserService {
    pub fn new(store: Arc<Store>) -> Self {
        Self { store }
    }

    pub fn get(&self, id: String) {
        let user = self.store.db.get(id.as_bytes()).unwrap().unwrap();
        let decode: User = bincode::deserialize(&user).unwrap();

        println!("{:?}", decode);
    }

    pub fn create(&self, user_dto: CreateUserDto) -> String {
        let id = self.store.generate_id();
        let mut user = User::from(user_dto);
        user.id = id;
        let encoded = bincode::serialize(&user).unwrap();
        self.store.db.insert(&user.id, encoded).unwrap();

        println!("User created!");
        return user.id;
    }
}
