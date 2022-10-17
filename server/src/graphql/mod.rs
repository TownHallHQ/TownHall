mod user;

use async_graphql::{Context, EmptySubscription, MergedObject, Object, Schema};

use self::user::UserQueryRoot;
use crate::context::SharedContext;
use crate::services::auth::Token;

pub struct MutationRoot;

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

// #[Object]
// impl QueryRoot {
//     async fn token_verify<'a>(&self, ctx: &'a Context<'_>) -> Option<i32> {
//         let context = ctx.data_unchecked::<SharedContext>();

//         if let Some(jwt) = ctx.data_opt::<Token>() {
//             let claims = context.services.auth.verify_token(jwt).unwrap();

//             return Some(claims.uid);
//         }

//         None
//     }
// }
