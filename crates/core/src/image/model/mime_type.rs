use serde::{Deserialize, Serialize};

pub const IMAGE_JPEG: &str = "image/jpeg";
pub const IMAGE_PNG: &str = "image/png";

/// Image file MIME Types
///
/// # Reference
///
/// - [MIME Types][1]
///
/// [1]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types
#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum MimeType {
    /// Joint Photographic Expert Group image (JPEG)
    Jpeg,
    /// Portable Network Graphics (PNG)
    Png,
}

impl ToString for MimeType {
    fn to_string(&self) -> String {
        match *self {
            MimeType::Jpeg => IMAGE_JPEG.to_string(),
            MimeType::Png => IMAGE_PNG.to_string(),
        }
    }
}
