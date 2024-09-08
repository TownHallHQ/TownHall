pub mod home;
pub mod login;
pub mod signup;

use leptos::{component, create_rw_signal, provide_context, view, IntoView};
use leptos_meta::{provide_meta_context, Title};
use leptos_router::{Route, Router, Routes};

use crate::components::layout::app::AppLayout;
use crate::components::layout::auth::AuthLayout;
use crate::context::Context;

use self::home::Home;
use self::login::Login;
use self::signup::SignUp;

#[component]
pub fn View() -> impl IntoView {
    provide_context(create_rw_signal(Context::default()));
    provide_meta_context();

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
