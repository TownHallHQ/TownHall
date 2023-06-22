use async_graphql::{EmptySubscription, Schema};

pub use super::modules::{MutationRoot, QueryRoot};

pub type GraphQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
