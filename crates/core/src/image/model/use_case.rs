use serde::{Deserialize, Serialize};

pub const AVATAR_MAX_SIZE: usize = 200_000;
pub const POST_IMAGE_MAX_SIZE: usize = 500_000;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum UseCase {
    /// Profile Picture for a user instance
    Avatar,
    /// Image associated to a Post
    Post,
}

impl UseCase {
    pub fn is_too_big(&self, size: usize) -> bool {
        if size > self.max_size_allowed() {
            return true;
        }

        false
    }

    pub fn max_size_allowed(&self) -> usize {
        match self {
            Self::Avatar => AVATAR_MAX_SIZE,
            Self::Post => POST_IMAGE_MAX_SIZE,
        }
    }
}

impl ToString for UseCase {
    fn to_string(&self) -> String {
        match *self {
            UseCase::Avatar => "avatar".to_string(),
            UseCase::Post => "post_image".to_string(),
        }
    }
}
