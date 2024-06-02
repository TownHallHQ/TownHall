use std::ops::Deref;
use std::str::FromStr;

use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use serde::{Deserialize, Serialize};

use townhall::user::model::Email as CoreEmail;

use crate::graphql::modules::user::types::{UserError, UserErrorCode};

/// [`Email`] formatted `String`
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Email(CoreEmail);

impl Email {
    pub fn into_inner(self) -> CoreEmail {
        self.0
    }
}

impl FromStr for Email {
    type Err = UserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match CoreEmail::from_str(s) {
            Ok(email) => Ok(Self(email)),
            Err(error) => Err(UserError {
                code: UserErrorCode::InvalidEmail,
                message: error.to_string(),
            }),
        }
    }
}

impl From<CoreEmail> for Email {
    fn from(value: CoreEmail) -> Self {
        Self(value)
    }
}

impl Deref for Email {
    type Target = CoreEmail;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[Scalar]
impl ScalarType for Email {
    fn parse(data: Value) -> InputValueResult<Self> {
        match data.clone() {
            Value::String(inner) => {
                if let Ok(email) = CoreEmail::from_str(&inner) {
                    return Ok(Self(email));
                }

                Err(InputValueError::expected_type(data))
            }
            _ => Err(InputValueError::expected_type(data)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}

impl ToString for Email {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[cfg(test)]
mod tests {
    use async_graphql::ScalarType;

    use super::{Email, Value};

    #[test]
    fn validates_string_is_actual_email_instance() {
        let email_str = String::from("joseph@uc.edu");
        let stri_value = Value::String(email_str);
        let email_scalar = Email::parse(stri_value).unwrap();
        let value = email_scalar.to_string();

        assert_eq!(value, "joseph@uc.edu");
    }

    #[test]
    fn invalidates_string_not_email() {
        let email_str = String::from("Joseph at UC University");
        let stri_value = Value::String(email_str);
        let email_scalar = Email::parse(stri_value);

        assert!(email_scalar.is_err());
    }
}
