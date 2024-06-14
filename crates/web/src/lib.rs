pub mod components;
pub mod views;

use leptos::{component, view, IntoView};
use leptos_meta::{provide_meta_context, Title};
use leptos_router::{Route, Router, Routes};

use crate::views::home::Home;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="TownHall"/>
        <Router>
            <Routes>
                <Route path="/" view=Home/>
            </Routes>
        </Router>
    }
}
