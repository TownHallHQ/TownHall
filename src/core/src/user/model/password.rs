use std::borrow::Cow;
use std::str::FromStr;

use argon2::{hash_encoded, verify_encoded, Config};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const PASSWORD_MIN_AMOUNT_OF_DIGITS: usize = 1;
pub const PASSWORD_MIN_LENGTH: usize = 8;
pub const PASSWORD_MIN_LOWERCASE_CHARS: usize = 1;
pub const PASSWORD_MIN_UPPERCASE_CHARS: usize = 1;
pub const PASSWORD_MIN_SYMBOLS: usize = 1;
pub const PASSWORD_SYMBOLS_REGEXP: &str = r#"[$-/:-?{-~!"^_`\[\]]"#;

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum PasswordError {
    #[error("A password must have at least {PASSWORD_MIN_AMOUNT_OF_DIGITS} digits. This password has {0} digits characters.")]
    MissingDigits(usize),
    #[error("A password must have at least {PASSWORD_MIN_LOWERCASE_CHARS} lowercase characters. This password has {0} lowercase characters.")]
    MissingLowerCaseChars(usize),
    #[error("A password must have at least {PASSWORD_MIN_UPPERCASE_CHARS} uppercase characters. This password has {0} uppercase characters.")]
    MissingUpperCaseChars(usize),
    #[error("A password must have at least {PASSWORD_MIN_SYMBOLS} symbols. This password has {0} symbols.")]
    MissingSymbols(usize),
    #[error("A password must have at least {PASSWORD_MIN_LENGTH} characters long. This password is {0} long.")]
    TooShort(usize),
    #[error("Failed to hash password")]
    HashEncodingError,
    #[error("Failed to verify password")]
    HashVerifyError,
}

/// Password [Value Object][1]
///
/// # Validation
///
/// [1]: https://en.wikipedia.org/wiki/Value_object
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Password(pub(crate) Cow<'static, str>);

impl ToString for Password {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Password {
    fn matches(re: &str, text: &str) -> usize {
        if let Some(caps) = Regex::new(re).unwrap().captures(text) {
            return caps.len();
        }

        0
    }

    /// Hashes the provided string
    pub fn hash(raw: &str) -> Result<String, PasswordError> {
        let salt: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();

        hash_encoded(raw.as_bytes(), salt.as_bytes(), &Config::default()).map_err(|err| {
            tracing::error!("Failed to hash password. {err}");

            PasswordError::HashEncodingError
        })
    }

    pub fn random() -> Self {
        let random: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();

        Self(random.into())
    }

    /// Checks if the provided raw password corresponds to the internal hash
    pub fn check(&self, raw: &str) -> Result<bool, PasswordError> {
        verify_encoded(&self.0, raw.as_bytes()).map_err(|err| {
            tracing::error!("Failed to verify password. {err}");

            PasswordError::HashVerifyError
        })
    }
}

impl FromStr for Password {
    type Err = PasswordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() || s.len() < PASSWORD_MIN_LENGTH {
            return Err(PasswordError::TooShort(s.len()));
        }

        let uppercase_len = Self::matches("[A-Z]", s);
        if uppercase_len < PASSWORD_MIN_UPPERCASE_CHARS {
            return Err(PasswordError::MissingUpperCaseChars(uppercase_len));
        }

        let lowercase_len = Self::matches("[a-z]", s);
        if lowercase_len < PASSWORD_MIN_LOWERCASE_CHARS {
            return Err(PasswordError::MissingLowerCaseChars(lowercase_len));
        }

        let digits_len = Self::matches("[0-9]", s);
        if digits_len < PASSWORD_MIN_AMOUNT_OF_DIGITS {
            return Err(PasswordError::MissingDigits(digits_len));
        }

        let symbols_len = Self::matches(PASSWORD_SYMBOLS_REGEXP, s);
        if symbols_len < PASSWORD_MIN_SYMBOLS {
            return Err(PasswordError::MissingSymbols(symbols_len));
        }

        let hashed = Password::hash(s)?;
        let cow = Cow::from(hashed);

        Ok(Self(cow))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{Password, PasswordError};

    #[test]
    fn creates_password_from_str() {
        let password = "ThisIs4G00dP4$$w0Rd";
        let have = Password::from_str(password).unwrap();

        assert!(have.0.starts_with("$argon2"));
    }

    #[test]
    fn validates_password_successfully() {
        let raw_password = "ok_h3reW3g04g%in";
        let have = Password::from_str(raw_password).unwrap();

        assert!(have.check(raw_password).unwrap());
    }

    #[test]
    fn validates_password_is_invalid_successfully() {
        let raw_password = "ok_h3reW3g04g%in";
        let have = Password::from_str(raw_password).unwrap();

        assert!(!have.check("thisisnotmypassword").unwrap());
    }

    #[test]
    fn checks_for_valid_passwords() {
        let passwords = vec!["ThisIs4G00dP4$$w0Rd", "this_is_a_good_p4$w0Rd", "1$0mAz_1"];

        for password in passwords {
            assert!(Password::from_str(password).is_ok(), "password: {password}");
        }
    }

    #[test]
    fn checks_for_invalid_passwords() {
        let passwords = vec![
            (
                "thisisaninvalidpassword",
                Err(PasswordError::MissingUpperCaseChars(0)),
            ),
            (
                "ThisIsAnInvalidPassword",
                Err(PasswordError::MissingDigits(0)),
            ),
            ("tH1$iS8", Err(PasswordError::TooShort(7))),
            (
                "ALL_CAPS_DOESNT_WORK_NEITHER",
                Err(PasswordError::MissingLowerCaseChars(0)),
            ),
            (
                "ALL_CAPS_DOESNT_WORK_NEITHER_1",
                Err(PasswordError::MissingLowerCaseChars(0)),
            ),
            (
                "AllCapsDoesntWorkNeither1",
                Err(PasswordError::MissingSymbols(0)),
            ),
        ];

        for (have, want) in passwords {
            assert_eq!(Password::from_str(have), want, "password: {have}");
        }
    }
}
