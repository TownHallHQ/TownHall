use std::borrow::Cow;
use std::fmt::Display;
use std::str::FromStr;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::link::error::LinkError;

pub const ULID_LEN: usize = 16;

/// A ULID is a unique identifier for a link. It must be URL safe.
/// ULID stands for:
/// - Unique
/// - Link
/// - Identifier
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Ulid(Cow<'static, str>);

impl FromStr for Ulid {
    type Err = LinkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(LinkError::UlidParseError(s.to_string()));
        }

        let s = s.trim().replace(' ', "-");

        for c in s.chars() {
            if !c.is_ascii_alphanumeric() && c != '-' && c != '_' {
                return Err(LinkError::UlidParseError(s.to_string()));
            }
        }

        Ok(Ulid(Cow::from(s)))
    }
}

impl Display for Ulid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for Ulid {
    fn default() -> Self {
        let val: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(ULID_LEN)
            .map(char::from)
            .collect();

        Ulid(val.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ulid_from_str() {
        let ulid = Ulid::from_str(" abc 123 ").unwrap();
        assert_eq!(ulid.0, "abc-123");

        let ulid = Ulid::from_str("abc123").unwrap();
        assert_eq!(ulid.0, "abc123");

        let ulid = Ulid::from_str("abc-123").unwrap();
        assert_eq!(ulid.0, "abc-123");

        let ulid = Ulid::from_str("abc_123").unwrap();
        assert_eq!(ulid.0, "abc_123");

        let ulid = Ulid::from_str("abc.123");
        assert!(ulid.is_err());

        let ulid = Ulid::from_str("abc 123");
        assert!(ulid.is_err());
    }
}
