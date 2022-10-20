mod user;

use async_graphql::{Context, EmptySubscription, MergedObject, Object, Schema};

use self::user::{UserMutationRoot, UserQueryRoot};
use crate::context::SharedContext;

#[derive(MergedObject, Default)]
pub struct MutationRoot(pub UserMutationRoot);

#[derive(MergedObject, Default)]
pub struct QueryRoot(pub UserQueryRoot);

pub type GraphQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[Object]
impl MutationRoot {
    async fn token_create<'a>(&self, ctx: &'a Context<'_>) -> Option<String> {
        let context = ctx.data_unchecked::<SharedContext>();
        let token = context.services.auth.sign_token(1).unwrap();

        Some(token.to_string())
    }
}
