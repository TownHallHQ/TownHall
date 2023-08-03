use std::str::FromStr;

use async_graphql::connection::{query, Connection, Edge, EmptyFields};
use async_graphql::{Context, Result};
use gabble::shared::pagination::Pagination;
use gabble::user::model::{Email, Username};
use gabble::user::repository::UserFilter;
use pxid::graphql::Pxid;

use crate::context::SharedContext;
use crate::graphql::connection_details::ConnectionDetails;
use crate::graphql::modules::user::types::User;

pub type UsersConnection = Connection<Pxid, Users, ConnectionDetails, EmptyFields>;

pub struct Users;

impl Users {
    pub async fn exec(
        ctx: &Context<'_>,
        after: Option<Pxid>,
        before: Option<Pxid>,
        first: Option<i32>,
        last: Option<i32>,
        id: Option<Pxid>,
        email: Option<String>,
        username: Option<String>,
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
                let filter = UserFilter {
                    id: id.and_then(|i| Some(i.into_inner())),
                    email: email.and_then(|s| Email::from_str(&s).ok()),
                    username: username.and_then(|s| Username::from_str(&s).ok()),
                };
                let query_set = context
                    .services
                    .user
                    .list(Some(pagination), Some(filter))
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
                        Ok(p) => Some(Edge::new(u.id, u)),
                        Err(err) => {
                            tracing::error!(%err, "Failed to create post instance from result");
                            None
                        }
                    }
                }));

                Ok::<UsersConnection, async_graphql::Error>(connection)
            },
        )
        .await;

        todo!()
    }
}
