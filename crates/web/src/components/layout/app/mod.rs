mod header;

use leptos::{
    component, create_render_effect, expect_context, spawn_local, view, IntoView,
    SignalGetUntracked,
};
use leptos_router::{use_navigate, Outlet};

use crate::context::{session::UserSession, AppContext};

use self::header::Header;

#[component]
pub fn AppLayout() -> impl IntoView {
    let app_context = expect_context::<AppContext>();

    create_render_effect(move |_| {
        spawn_local(async move {
            if let UserSession::Unauthenticated =
                app_context.get_untracked().session.user.get_untracked()
            {
                let navigate = use_navigate();
                navigate("/auth/login", Default::default());
            }
        });
    });

    view! {
        <div class="bg-gray-50 min-h-screen">
            <Header />
            <main class="grid md:grid-cols-12 md:max-w-[1400px] px-4 py-2 mx-auto">
                <Outlet />
            </main>
        </div>
    }
}
