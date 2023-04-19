mod link;

use async_graphql::MergedObject;

use self::link::mutation::LinkMutationRoot;

#[derive(MergedObject, Default)]
pub struct MutationRoot(LinkMutationRoot);

#[derive(MergedObject, Default)]
pub struct QueryRoot;
