use std::borrow::Cow;
use std::fmt::{self, Display};
use std::str::FromStr;

use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const USERNAME_REGEXP: &str = r#"^[a-zA-Z][a-zA-Z0-9_]*[a-zA-Z0-9]$"#;

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum UsernameError {
    #[error("The provided username is not a valid username: {0}")]
    ParseError(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Username(pub(crate) Cow<'static, str>);

impl Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Username {
    type Err = UsernameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(UsernameError::ParseError(s.to_string()));
        }

        if s.contains(' ') {
            return Err(UsernameError::ParseError(s.to_string()));
        }

        let username_regex = Regex::new(USERNAME_REGEXP).unwrap();

        if !username_regex.is_match(s) {
            return Err(UsernameError::ParseError(s.to_string()));
        }

        let cow = Cow::from(s.to_string());
        return Ok(Self(cow));
    }
}

mod tests {
    use std::borrow::Cow;
    use std::str::FromStr;

    use super::Username;

    #[test]
    fn creates_username_from_str() {
        let username = "johndoe";
        let have = Username::from_str(username).unwrap();
        let want = Username(Cow::from(username));

        assert_eq!(have, want);
    }

    #[test]
    fn checks_for_valid_username() {
        let usernames = vec!["john", "john_doe", "john_doe_appleseed", "john12", "j0hn"];

        for username in usernames {
            assert!(Username::from_str(username).is_ok(), "username: {username}");
        }
    }

    #[test]
    fn checks_for_invalid_usernames() {
        let usernames = vec![
            "",
            " ",
            "john doe",
            "#@%^%#$@#$@#",
            "あいうえお",
            "_",
            "_john_",
            "45algo",
            "123",
        ];

        for username in usernames {
            let result = Username::from_str(username);

            assert!(result.is_err(), "username: {username}")
        }
    }
}
