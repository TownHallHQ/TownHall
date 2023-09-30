pub mod auth;
pub mod post;
pub mod user;

use async_graphql::MergedObject;

use self::auth::mutation::AuthMutationRoot;
use self::auth::query::AuthQueryRoot;
use self::post::mutation::PostMutationRoot;
use self::post::query::PostQueryRoot;
use self::user::mutation::UserMutationRoot;
use self::user::query::UserQueryRoot;

#[derive(MergedObject, Default)]
pub struct MutationRoot(AuthMutationRoot, UserMutationRoot, PostMutationRoot);

#[derive(MergedObject, Default)]
pub struct QueryRoot(AuthQueryRoot, UserQueryRoot, PostQueryRoot);
