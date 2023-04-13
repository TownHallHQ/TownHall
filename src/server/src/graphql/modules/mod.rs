use async_graphql::{EmptySubscription, MergedObject, Schema, SimpleObject, ID};

#[derive(MergedObject, Default)]
pub struct MutationRoot;

#[derive(MergedObject, Default)]
pub struct QueryRoot;

pub type GraphQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
