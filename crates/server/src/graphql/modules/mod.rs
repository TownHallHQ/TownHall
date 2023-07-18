mod post;
mod user;

use async_graphql::MergedObject;

use self::post::mutation::PostMutationRoot;
use self::user::mutation::UserMutationRoot;
use self::user::query::UserQueryRoot;

#[derive(MergedObject, Default)]
pub struct MutationRoot(UserMutationRoot, PostMutationRoot);

#[derive(MergedObject, Default)]
pub struct QueryRoot(UserQueryRoot);
