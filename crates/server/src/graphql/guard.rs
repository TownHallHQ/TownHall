use async_graphql::{Context, Error, ErrorExtensions, Guard, Result};
use async_trait::async_trait;

use crate::services::auth::Token;

const GENERIC_FORBIDDEN_ERROR: &str = "You must authenticate to perform this request. Use the `tokenCreate` mutation and set the token as the Authorization HTTP Header E.g. `JWT <TOKEN>`";
const GENERIC_FORBIDDEN_ERROR_CODE: &str = "UNAUTHORIZED";

/// GraphQL guard to check if the user is authenticated by checking if the
/// token is present in the context.
#[derive(Clone, Debug, Default)]
pub struct AuthenticationGuard;

impl AuthenticationGuard {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Guard for AuthenticationGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        if ctx.data_opt::<Token>().is_some() {
            return Ok(());
        }

        let forbidden_error = Error::new(GENERIC_FORBIDDEN_ERROR)
            .extend_with(|_, e| e.set("code", GENERIC_FORBIDDEN_ERROR_CODE));

        Err(forbidden_error)
    }
}
