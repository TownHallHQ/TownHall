use std::sync::Arc;

use async_graphql::{EmptySubscription, Schema};

use crate::context::Context;

pub use super::modules::{MutationRoot, QueryRoot};

pub type GraphQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn build_schema_with_context(context: &Arc<Context>) -> GraphQLSchema {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(Arc::clone(context))
    .finish()
}
