use std::borrow::Cow;
use std::fmt::{self, Display};
use std::str::FromStr;

use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// A regexp that represents a valid email identifier
///
/// Refer: https://i.stack.imgur.com/YI6KR.png
pub const EMAIL_IDENTIFIER_REGEXP: &str = r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*)"#;

/// A regexp that represents a valid email domain
///
/// Refer: https://i.stack.imgur.com/YI6KR.png
pub const EMAIL_DOMAIN_REGEXP: &str = r#"(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#;

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum EmailError {
    #[error("The provided email domain \"{0}\" on email \"{1}\" is not valid")]
    InvalidDomain(String, String),
    #[error("The provided email identifier \"{0}\" on email \"{1}\" is not valid")]
    InvalidIdentifier(String, String),
    #[error("The provided email is not a valid email: {0}")]
    ParseError(String),
}

/// Email value
///
/// # Validation
///
/// Email validation is achieved using Regular Expressions and certain rules.
/// Refer: https://stackoverflow.com/questions/201323/how-can-i-validate-an-email-address-using-a-regular-expression
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Email(pub(crate) Cow<'static, str>);

impl Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl FromStr for Email {
    type Err = EmailError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(EmailError::ParseError(s.to_string()));
        }

        if s.contains(' ') {
            return Err(EmailError::ParseError(s.to_string()));
        }

        let parts = s.split('@').collect::<Vec<&str>>();

        if parts.len() > 2 {
            return Err(EmailError::ParseError(s.to_string()));
        }

        if let (Some(ident), Some(domain)) = (parts.first(), parts.get(1)) {
            let ident_regex = Regex::new(EMAIL_IDENTIFIER_REGEXP).unwrap();
            let domain_regex = Regex::new(EMAIL_DOMAIN_REGEXP).unwrap();

            if !ident_regex.is_match(ident) {
                return Err(EmailError::InvalidIdentifier(
                    ident.to_string(),
                    s.to_string(),
                ));
            }

            if !domain_regex.is_match(domain) {
                return Err(EmailError::InvalidDomain(domain.to_string(), s.to_string()));
            }

            let cow = Cow::from(s.to_string());

            return Ok(Self(cow));
        }

        Err(EmailError::ParseError(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use std::str::FromStr;

    use super::Email;

    #[test]
    fn creates_email_from_str() {
        let email = "john.appleseed@example.com";
        let have = Email::from_str(email).unwrap();
        let want = Email(Cow::from(email));

        assert_eq!(have, want);
    }

    #[test]
    fn checks_for_valid_emails() {
        let emails = vec![
            "email@example.com",
            "firstname.lastname@example.com",
            "email@subdomain.example.com",
            "firstname+lastname@example.com",
            "email@123.123.123.123",
            "email@[123.123.123.123]",
            "1234567890@example.com",
            "email@example-one.com",
            "_______@example.com",
            "email@example.name",
            "email@example.museum",
            "email@example.co.jp",
            "firstname-lastname@example.com",
        ];

        for email in emails {
            assert!(Email::from_str(email).is_ok(), "email: {email}");
        }
    }

    #[test]
    fn checks_for_invalid_emails() {
        let emails = vec![
            "plainaddress",
            "#@%^%#$@#$@#.com",
            "@example.com",
            "Joe Smith <email@example.com>",
            "email.example.com",
            "email@example@example.com",
            "あいうえお@example.com",
            "email@example.com (Joe Smith)",
            "email@example",
            "email@example..com",
        ];

        for email in emails {
            let result = Email::from_str(email);

            assert!(result.is_err(), "email: {email}");
        }
    }
}
