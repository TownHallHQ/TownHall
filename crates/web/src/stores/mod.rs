pub mod session;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AppStore {
    pub session: session::SessionStore,
}
