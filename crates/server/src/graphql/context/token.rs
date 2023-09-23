use anyhow::Result;
use axum::http::header::AUTHORIZATION;
use axum::http::HeaderMap;

use crate::services::auth::{AuthService, Token};

const HTTP_AUTHORIZATION_SCHEME: &str = "jwt";

pub fn try_extract_token(auth_service: &AuthService, headers: &HeaderMap) -> Result<Option<Token>> {
    if let Some(jwt) = get_jwt_from_auth_header(headers) {
        let token = auth_service.parse_jwt(&jwt).ok();

        match token {
            Some(token) => {
                return Ok(Some(token));
            }
            None => {
                anyhow::bail!("Failed to parse JWT");
            }
        }
    }

    Ok(None)
}

/// Retrieves the value from the Authorization HTTP Header and attempts to
/// retrieve the JSON Web Token if the Authorization Scheme is `JWT` and a
/// token is present.
fn get_jwt_from_auth_header(headers: &HeaderMap) -> Option<String> {
    if let Some(authorization) = headers.get(AUTHORIZATION) {
        let value = authorization.to_str().unwrap();
        let parts = value.split(' ').collect::<Vec<&str>>();

        if let (Some(scheme), Some(token)) = (parts.first(), parts.get(1)) {
            let scheme = scheme.to_string();
            let token = token.to_string();

            if scheme.to_ascii_lowercase() == HTTP_AUTHORIZATION_SCHEME && token.len() > 1 {
                return Some(token);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use axum::http::{HeaderMap, HeaderValue};

    use super::{get_jwt_from_auth_header, AUTHORIZATION};

    #[test]
    fn gets_the_token_from_the_http_header() {
        let mut headers = HeaderMap::new();
        let auth_header = HeaderValue::from_static("JWT 12345");

        headers.append(AUTHORIZATION, auth_header);

        let token = get_jwt_from_auth_header(&headers);

        assert!(token.is_some());

        let token = token.unwrap();

        assert_eq!(token, "12345");
    }

    #[test]
    fn returns_none_if_the_token_has_other_scheme() {
        let mut headers = HeaderMap::new();
        let auth_header = HeaderValue::from_static("Bearer 12345");

        headers.append(AUTHORIZATION, auth_header);

        let token = get_jwt_from_auth_header(&headers);

        assert!(token.is_none());
    }
}
