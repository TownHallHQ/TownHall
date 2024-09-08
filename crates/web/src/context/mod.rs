pub mod session;

use leptos::RwSignal;

pub type AppContext = RwSignal<Context>;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Context {
    pub session: session::SessionContext,
}
