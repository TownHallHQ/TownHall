use async_graphql::Object;

#[derive(Debug, Default)]
pub struct LinkQueryRoot;

#[Object]
impl LinkQueryRoot {
    async fn link_list(&self) -> bool {
        println!("LinkQueryRoot::link_list");

        true
    }
}
