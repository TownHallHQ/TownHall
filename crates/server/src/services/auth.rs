use anyhow::{Error, Result};
use argon2::verify_encoded;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use pxid::Pxid;
use serde::{Deserialize, Serialize};

const JWT_AUDIENCE: &str = "townhall";

/// JWT Token Abstaction
#[derive(Debug)]
pub struct Token {
    pub(crate) raw: String,
    pub(crate) claims: Claims,
}

impl Token {
    /// Retrieves the token's user ID
    pub fn user_id(&self) -> Pxid {
        self.claims.uid
    }

    /// Retrieves the internal JWT String
    pub fn token_string(&self) -> String {
        self.raw.to_string()
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        self.raw.clone()
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

        Ok(Token { raw: jwt, claims })
    }

    pub fn verify_token(&self, token: &Token) -> Result<Claims> {
        let token_data = decode::<Claims>(&token.raw, &self.decoding_key, &self.validation)
            .map_err(|e| Error::msg(e.to_string()))?;

        Ok(token_data.claims)
    }

    pub fn validate_password(&self, encoded: &str, raw: &str) -> bool {
        let raw = raw.as_bytes();

        verify_encoded(encoded, raw).unwrap()
    }

    pub fn parse_jwt(&self, jwt: &str) -> Result<Token> {
        let claims = Self::decode_token(jwt, &self.decoding_key, &self.validation)?;

        Ok(Token {
            raw: jwt.to_string(),
            claims,
        })
    }

    pub(crate) fn decode_token(
        token: &str,
        decoding_key: &DecodingKey,
        validation: &Validation,
    ) -> Result<Claims> {
        let token_data = decode::<Claims>(token, decoding_key, validation)?;

        Ok(token_data.claims)
    }
}
