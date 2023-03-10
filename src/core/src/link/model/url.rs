use std::borrow::Cow;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::link::error::{LinkError, Result};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Url(Cow<'static, str>);

impl ToString for Url {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl FromStr for Url {
    type Err = LinkError;

    fn from_str(s: &str) -> Result<Self> {
        use url::Url;

        let url = Url::from_str(s).map_err(LinkError::from)?;

        Ok(Self(Cow::from(url.to_string())))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use url::ParseError;

    use crate::link::error::LinkError;

    use super::Url;

    #[test]
    fn parses_url_from_str() {
        let url_str = "https://example.com/";
        let url_ins = Url::from_str(url_str).unwrap();

        assert_eq!(url_str, url_ins.0);
    }

    #[test]
    fn validates_url_string() {
        let url_str = "thisisnotaurl";
        let url_ins = Url::from_str(url_str);

        assert!(url_ins.is_err());
        assert_eq!(
            url_ins,
            Err(LinkError::ParseUrlError(ParseError::RelativeUrlWithoutBase))
        );
    }

    #[test]
    fn retrieves_string_value() {
        let url_str = "https://example.com/";
        let url_ins = Url::from_str(url_str).unwrap();

        assert_eq!(url_str, url_ins.to_string());
    }
}
