use async_graphql::Object;

#[derive(Debug, Default)]
pub struct LinkMutationRoot;

#[Object]
impl LinkMutationRoot {
    async fn link_create(&self) -> bool {
        println!("LinkMutationRoot::link_create");

        true
    }
}
