pub mod components;
pub mod stores;
pub mod views;

use leptos::{component, create_rw_signal, provide_context, view, IntoView};
use leptos_meta::{provide_meta_context, Title};
use leptos_router::{Route, Router, Routes};

use crate::components::layout::Layout;
use crate::stores::AppStore;
use crate::views::home::Home;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_context(create_rw_signal(AppStore::default()));

    view! {
        <Title text="TownHall"/>
        <Router>
            <Routes>
                <Route path="/" view={Layout}>
                    <Route path="/" view=Home />
                </Route>
            </Routes>
        </Router>
    }
}
