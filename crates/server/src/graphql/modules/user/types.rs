use async_graphql::connection::{query, Connection, Edge, EmptyFields};
use async_graphql::{ComplexObject, Context, Enum, Result, SimpleObject};
use chrono::{DateTime, Utc};
use gabble::post::repository::PostFilter;
use gabble::shared::pagination::Pagination;
use pxid::graphql::Pxid;
use serde::{Deserialize, Serialize};

use crate::context::SharedContext;
use crate::graphql::connection_details::ConnectionDetails;
use crate::graphql::modules::post::types::Post;

pub type UserPostsConnection = Connection<Pxid, Post, ConnectionDetails, EmptyFields>;

#[derive(Copy, Clone, Debug, Deserialize, Enum, Eq, PartialEq, Serialize)]
pub enum UserErrorCode {
    InvalidEmail,
    EmailTaken,
    Unauthorized,
    Internal,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct AccessToken {
    pub access_token: String,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct UserError {
    pub code: UserErrorCode,
    pub message: String,
}

/// A Platform's User
#[derive(Debug, Deserialize, Serialize, SimpleObject)]
#[graphql(complex)]
pub struct User {
    pub id: Pxid,
    pub name: String,
    pub surname: String,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[ComplexObject]
impl User {
    /// Lists posts authored by this user
    async fn posts(
        &self,
        ctx: &Context<'_>,
        after: Option<Pxid>,
        before: Option<Pxid>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<UserPostsConnection> {
        let context = ctx.data_unchecked::<SharedContext>();
        let after = after.map(|a| a.to_string());
        let before = before.map(|a| a.to_string());

        query(
            after,
            before,
            first,
            last,
            |after: Option<Pxid>,
             before: Option<Pxid>,
             first: Option<usize>,
             last: Option<usize>| async move {
                let pagination = Pagination::new(
                    after.map(|id| id.into_inner()),
                    before.map(|id| id.into_inner()),
                    first,
                    last,
                )?;
                let query_set = context
                    .services
                    .post
                    .list(
                        Some(pagination),
                        Some(PostFilter {
                            author_id: Some(self.id.into_inner()),
                        }),
                    )
                    .await?;
                let total_count = query_set.count();
                let posts = query_set.records();
                let page_info = pagination.get_page_info(total_count);
                let mut connection = UserPostsConnection::with_additional_fields(
                    page_info.has_prev_pages,
                    page_info.has_next_pages,
                    ConnectionDetails { total_count },
                );

                connection.edges.extend(posts.into_iter().filter_map(|p| {
                    match Post::try_from(p) {
                        Ok(p) => Some(Edge::new(p.id, p)),
                        Err(err) => {
                            tracing::error!(%err, "Failed to create post instance from result");
                            None
                        }
                    }
                }));

                Ok::<UserPostsConnection, async_graphql::Error>(connection)
            },
        )
        .await
    }
}

impl From<gabble::user::model::User> for User {
    fn from(value: gabble::user::model::User) -> Self {
        User {
            id: value.id.into(),
            name: value.name,
            surname: value.surname,
            username: value.username.to_string(),
            email: value.email.to_string(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
