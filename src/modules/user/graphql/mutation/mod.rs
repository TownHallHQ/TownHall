use async_graphql::Object;

#[derive(Debug, Default)]
pub struct UserMutationRoot;

#[Object]
impl UserMutationRoot {
    async fn user_create(&self) -> bool {
        println!("UserMutationRoot::user_create");
        true
    }
}
