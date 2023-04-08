use std::borrow::Cow;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::link::error::LinkError;

/// A Fingerprint is a unique identifier for a link. It must be URL safe.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Fingerprint(Cow<'static, str>);

impl FromStr for Fingerprint {
    type Err = LinkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(LinkError::FingerprintParseError(s.to_string()));
        }

        let s = s.trim().replace(' ', "-");

        for c in s.chars() {
            if !c.is_ascii_alphanumeric() && c != '-' && c != '_' {
                return Err(LinkError::FingerprintParseError(s.to_string()));
            }
        }

        Ok(Fingerprint(Cow::from(s.to_string())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fingerprint_from_str() {
        let fingerprint = Fingerprint::from_str(" abc 123 ").unwrap();
        assert_eq!(fingerprint.0, "abc-123");

        let fingerprint = Fingerprint::from_str("abc123").unwrap();
        assert_eq!(fingerprint.0, "abc123");

        let fingerprint = Fingerprint::from_str("abc-123").unwrap();
        assert_eq!(fingerprint.0, "abc-123");

        let fingerprint = Fingerprint::from_str("abc_123").unwrap();
        assert_eq!(fingerprint.0, "abc_123");

        let fingerprint = Fingerprint::from_str("abc.123");
        assert!(fingerprint.is_err());

        let fingerprint = Fingerprint::from_str("abc 123");
        assert!(fingerprint.is_err());
    }
}
