use leptos::{
    component, create_render_effect, create_slice, expect_context, spawn_local, view, IntoView,
    SignalGet, SignalGetUntracked,
};
use leptos_router::{use_navigate, Outlet};

use crate::context::session::UserSession;
use crate::AppContext;

#[component]
pub fn AuthLayout() -> impl IntoView {
    let app_context = expect_context::<AppContext>();
    let (session_getter, _) = create_slice(
        app_context,
        |app_ctx| app_ctx.session.user.get(),
        |_, _: ()| unimplemented!(),
    );

    create_render_effect(move |_| {
        let app_context = app_context;

        spawn_local(async move {
            let navigate = use_navigate();

            if app_context.get_untracked().session.whoami().await.is_ok() {
                if let UserSession::Authenticated(_) = session_getter.get() {
                    navigate("/", Default::default())
                }
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
