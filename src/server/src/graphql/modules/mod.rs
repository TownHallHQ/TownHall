mod link;
mod user;

use async_graphql::MergedObject;

use self::link::mutation::LinkMutationRoot;
use self::user::mutation::UserMutationRoot;
use self::user::query::UserQueryRoot;

#[derive(MergedObject, Default)]
pub struct MutationRoot(LinkMutationRoot, UserMutationRoot);

#[derive(MergedObject, Default)]
pub struct QueryRoot(UserQueryRoot);
