use std::str::FromStr;

use anyhow::{Error, Result};
use argon2::verify_encoded;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use pxid::Pxid;
use serde::{Deserialize, Serialize};

const JWT_AUDIENCE: &str = "quicklink";

/// JWT Token Abstaction
#[derive(Debug)]
pub struct Token(pub String);

impl ToString for Token {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl FromStr for Token {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');

        if let (Some(schema), Some(token)) = (parts.next(), parts.next()) {
            if schema.to_lowercase() == "jwt" && token.len() > 1 {
                return Ok(Token(token.to_string()));
            }
        }

        Err(Error::msg("Invalid JWT Token Provided. You must provide an `Authorization` header with a JWT token as `JWT <Base64 JWT Token>`"))
    }
}

#[derive(Clone)]
pub struct AuthService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validation: Validation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub uid: Pxid,
    pub iat: usize,
}

impl AuthService {
    pub fn new(jwt_secret: &str) -> Self {
        let encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());
        let mut validation = Validation::new(Algorithm::HS256);

        validation.set_audience(JWT_AUDIENCE.as_bytes());

        Self {
            encoding_key,
            decoding_key,
            validation,
        }
    }

    pub fn sign_token(&self, uid: Pxid) -> Result<Token> {
        let iat = Utc::now().timestamp() as usize;
        let exp = Utc::now()
            .checked_add_signed(Duration::days(30))
            .unwrap()
            .timestamp() as usize;
        let claims = Claims { exp, iat, uid };
        let jwt = encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| Error::msg(e.to_string()))?;

        Ok(Token(jwt))
    }

    pub fn verify_token(&self, token: &Token) -> Result<Claims> {
        let token_data = decode::<Claims>(&token.0, &self.decoding_key, &self.validation)
            .map_err(|e| Error::msg(e.to_string()))?;

        Ok(token_data.claims)
    }

    pub fn validate_password(&self, encoded: &str, raw: &str) -> bool {
        let raw = raw.as_bytes();

        verify_encoded(encoded, raw).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creates_token_from_string() {
        let auth_header_value = "jwt 1234";
        let token = Token::from_str(auth_header_value).unwrap();

        assert_eq!(token.0, String::from("1234"));
    }

    #[test]
    #[should_panic(
        expected = "Invalid JWT Token Provided. You must provide an `Authorization` header with a JWT token as `JWT <Base64 JWT Token>`"
    )]
    fn fails_with_arbitrary_string_provided() {
        let auth_header_value = "testing one23";

        Token::from_str(auth_header_value).unwrap();
    }
}
