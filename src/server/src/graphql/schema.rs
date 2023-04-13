use std::sync::Arc;

use async_graphql::extensions::Tracing;
use async_graphql::{EmptySubscription, Schema};

use crate::context::Context;

use super::modules::{MutationRoot, QueryRoot};

pub type GraphQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub type BuildSchemaContext = Context;

pub fn build_schema(context: &Arc<BuildSchemaContext>) -> GraphQLSchema {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .extension(Tracing)
    .data(Arc::clone(context))
    .finish()
}
