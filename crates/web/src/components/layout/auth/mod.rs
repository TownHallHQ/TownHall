use leptos::{
    component, create_render_effect, create_slice, expect_context, spawn_local, view, IntoView,
    SignalGet,
};
use leptos_router::{use_navigate, Outlet};

use crate::context::session::UserSession;
use crate::context::AppContext;

#[component]
pub fn AuthLayout() -> impl IntoView {
    let app_context = expect_context::<AppContext>();
    let (session_getter, _) =
        create_slice(app_context, |ctx| ctx.session.user.get(), |_, _: ()| {});

    create_render_effect(move |_| {
        let session = move || session_getter.get();

        spawn_local(async move {
            if let UserSession::Authenticated(_) = session() {
                let navigate = use_navigate();
                navigate("/", Default::default());
            }
        });
    });

    view! {
        <div>
            <main>
                <Outlet />
            </main>
        </div>
    }
}
