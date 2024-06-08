use async_graphql::{Context, Error, ErrorExtensions, Guard, Result};

use townhall::auth::service::Token;

use crate::context::SharedContext;

pub const GENERIC_FORBIDDEN_ERROR: &str = "You must authenticate to perform this request. Use the `tokenCreate` mutation and set the token as the Authorization HTTP Header E.g. `JWT <TOKEN>`";
pub const GENERIC_FORBIDDEN_ERROR_CODE: &str = "UNAUTHORIZED";

/// GraphQL guard to check if the user is authenticated by checking if the
/// token is present in the context.
#[derive(Clone, Debug, Default)]
pub struct AuthenticationGuard;

impl AuthenticationGuard {
    pub fn new() -> Self {
        Self
    }
}

impl Guard for AuthenticationGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let context = ctx.data_unchecked::<SharedContext>();

        if let Some(token) = ctx.data_opt::<Token>() {
            if context.services.auth.verify_token(token).is_ok() {
                return Ok(());
            }
        }

        Err(Error::new(GENERIC_FORBIDDEN_ERROR)
            .extend_with(|_, e| e.set("code", GENERIC_FORBIDDEN_ERROR_CODE)))
    }
}
