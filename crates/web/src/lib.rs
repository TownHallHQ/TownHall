pub mod components;
pub mod views;

use leptos::{component, view, IntoView};
use leptos_meta::{provide_meta_context, Title};
use leptos_router::{Route, Router, Routes};
use views::{login::Login, signup::Signup};

use crate::views::home::Home;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="TownHall"/>
        <Router>
            <Routes>
                <Route path="/" view=Home/>
                <Route path="/login" view=Login/>
                <Route path="/login" view=Signup/>
            </Routes>
        </Router>
    }
}
