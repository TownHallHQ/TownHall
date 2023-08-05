use std::str::FromStr;

use async_graphql::connection::{query, Connection, Edge, EmptyFields};
use async_graphql::{Context, InputObject, Result};
use pxid::graphql::Pxid;

use gabble::shared::pagination::Pagination;
use gabble::user::model::{Email, Username};
use gabble::user::repository::UserFilter;

use crate::context::SharedContext;
use crate::graphql::connection_details::ConnectionDetails;
use crate::graphql::modules::user::types::User;

pub type UsersConnection = Connection<Pxid, User, ConnectionDetails, EmptyFields>;

#[derive(Debug, Default, InputObject)]
pub struct UserFilterInput {
    pub id: Option<Pxid>,
    pub email: Option<String>,
    pub username: Option<String>,
}

impl From<UserFilterInput> for UserFilter {
    fn from(value: UserFilterInput) -> Self {
        UserFilter {
            id: value.id.map(|id| id.into_inner()),
            email: value.email.and_then(|s| Email::from_str(&s).ok()),
            username: value.username.and_then(|s| Username::from_str(&s).ok()),
        }
    }
}

pub struct Users;

impl Users {
    pub async fn exec(
        ctx: &Context<'_>,
        after: Option<Pxid>,
        before: Option<Pxid>,
        first: Option<i32>,
        last: Option<i32>,
        filter: Option<UserFilterInput>,
    ) -> Result<UsersConnection> {
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
                    .user
                    .list(Some(pagination), Some(filter.unwrap_or_default().into()))
                    .await?;
                let total_count = query_set.count();
                let users = query_set.records();
                let page_info = pagination.get_page_info(total_count);
                let mut connection = UsersConnection::with_additional_fields(
                    page_info.has_prev_pages,
                    page_info.has_next_pages,
                    ConnectionDetails { total_count },
                );

                connection.edges.extend(users.into_iter().filter_map(|u| {
                    match User::try_from(u) {
                        Ok(p) => Some(Edge::new(p.id, p)),
                        Err(err) => {
                            tracing::error!(%err, "Failed to create post instance from result");
                            None
                        }
                    }
                }));

                Ok::<UsersConnection, async_graphql::Error>(connection)
            },
        )
        .await
    }
}
