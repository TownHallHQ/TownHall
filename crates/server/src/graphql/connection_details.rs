use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct ConnectionDetails {
    pub total_count: u64,
}
