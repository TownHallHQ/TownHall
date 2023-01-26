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

    pub fn get_all(&self) -> Vec<User> {
        let user_tree = self.store.db.open_tree("users").unwrap();
        user_tree
            .iter()
            .map(|item| {
                let (_, v) = item.unwrap();
                bincode::deserialize::<User>(&v).unwrap()
            })
            .collect()
    }

    pub fn get(&self, id: String) {
        let user_tree = self.store.db.open_tree("users").unwrap();
        let user = user_tree.get(id.as_bytes()).unwrap().unwrap();
        let decode: User = bincode::deserialize(&user).unwrap();

        println!("{:?}", decode);
    }

    pub fn create(&self, user_dto: CreateUserDto) -> String {
        let user_tree = self.store.db.open_tree("users").unwrap();
        let id = self.store.generate_id();
        let mut user = User::from(user_dto);
        user.id = id;
        let encoded = bincode::serialize(&user).unwrap();
        user_tree.insert(&user.id, encoded).unwrap();

        println!("User created!");
        return user.id;
    }

    pub fn find_by_email(&self, email: String) -> Option<User> {
        let users = self.get_all();
        let all = users.into_iter().find(|item| email == item.email);

        all
    }
}
