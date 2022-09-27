use anyhow::Error;
use std::str::FromStr;

/// JWT Token Abstaction
pub struct Token(pub String);

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
    jwt_secret: String,
}

impl AuthService {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
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
