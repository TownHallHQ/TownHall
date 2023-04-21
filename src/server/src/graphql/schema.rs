use async_graphql::{EmptySubscription, Schema};

use super::modules::{MutationRoot, QueryRoot};

pub type GraphQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
