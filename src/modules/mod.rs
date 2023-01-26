// pub mod auth;
pub mod link;
pub mod user;

use async_graphql::{EmptySubscription, MergedObject, Schema};

// use self::auth::graphql::{AuthMutationRoot, AuthQueryRoot};
// use self::user::graphql::{UserMutationRoot, UserQueryRoot};
use self::link::graphql::{LinkMutationRoot, LinkQueryRoot};

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    pub LinkMutationRoot,
    // pub AuthMutationRoot,
    // pub UserMutationRoot,
);

#[derive(MergedObject, Default)]
// pub struct QueryRoot(pub AuthQueryRoot, pub UserQueryRoot);
pub struct QueryRoot(pub LinkQueryRoot);

pub type GraphQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
