use leptos::RwSignal;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SessionStore {
    pub user: RwSignal<Option<()>>,
}
