pub mod components;
pub mod stores;
pub mod views;

use leptos::{component, create_rw_signal, provide_context, view, IntoView};
use leptos_meta::{provide_meta_context, Title};
use leptos_router::{Route, Router, Routes};
use views::login::Login;
use views::signup::SignUp;

use crate::components::layout::app::AppLayout;
use crate::components::layout::auth::AuthLayout;
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
                <Route path="/" view={AppLayout}>
                    <Route path="/" view=Home />
                </Route>
                <Route path="/auth" view={AuthLayout}>
                    <Route path="/login" view=Login />
                    <Route path="/signup" view=SignUp />
                </Route>
            </Routes>
        </Router>
    }
}
