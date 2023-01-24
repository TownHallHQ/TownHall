use std::sync::Arc;

use crate::modules::auth::service::AuthService;

pub struct Services {
  auth: Arc<AuthService>
}
