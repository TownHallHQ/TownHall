// pub mod auth;
pub mod link;
pub mod user;

use async_graphql::{EmptySubscription, MergedObject, Schema};

// use self::auth::graphql::{AuthMutationRoot, AuthQueryRoot};
use self::link::graphql::{LinkMutationRoot, LinkQueryRoot};
use self::user::graphql::{UserMutationRoot, UserQueryRoot};

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    pub LinkMutationRoot,
    pub UserMutationRoot,
    // pub AuthMutationRoot,
);

#[derive(MergedObject, Default)]
// pub struct QueryRoot(pub AuthQueryRoot, pub UserQueryRoot);
pub struct QueryRoot(pub LinkQueryRoot, pub UserQueryRoot);

pub type GraphQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
