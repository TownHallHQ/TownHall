pub mod home;
pub mod login;
pub mod signup;

use leptos::{
    component, create_render_effect, expect_context, spawn_local, view, IntoView,
    SignalGetUntracked,
};
use leptos_meta::Title;
use leptos_router::{Route, Router, Routes};

use crate::components::layout::app::AppLayout;
use crate::components::layout::auth::AuthLayout;
use crate::context::session::UserSession;
use crate::context::AppContext;

use self::home::Home;
use self::login::Login;
use self::signup::SignUp;

#[component]
pub fn View() -> impl IntoView {
    let app_context = expect_context::<AppContext>();

    create_render_effect(move |_| {
        spawn_local(async move {
            let session = app_context.get_untracked().session.user.get_untracked();

            if let UserSession::Unknown = session {
                if let Err(err) = app_context.get_untracked().session.whoami().await {
                    leptos::logging::error!("Failed to check user session. {err}");
                }
            }
        });
    });

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
