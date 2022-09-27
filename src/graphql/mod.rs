use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};

use crate::services::auth::Token;

pub struct QueryRoot;

pub type GraphQLSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[Object]
impl QueryRoot {
    async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
        ctx.data_opt::<Token>().map(|token| token.0.as_str())
    }
}
