pub mod auth;
pub mod post;
pub mod user;

use async_graphql::{MergedObject, SimpleObject};
use pxid::Pxid;
use serde::{Deserialize, Serialize};

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

#[derive(Clone, Debug, SimpleObject, Serialize, Deserialize)]
pub struct Image {
    id: Pxid,
    url: String,
}

impl From<townhall::image::model::Image> for Image {
    fn from(value: townhall::image::model::Image) -> Self {
        Image {
            id: value.id,
            url: value.url,
        }
    }
}
