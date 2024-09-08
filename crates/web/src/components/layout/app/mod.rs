mod header;

use leptos::{
    component, create_render_effect, create_slice, expect_context, spawn_local, view, IntoView,
    SignalGet, SignalGetUntracked,
};
use leptos_router::{use_navigate, Outlet};

use crate::context::{session::UserSession, AppContext};

use self::header::Header;

#[component]
pub fn AppLayout() -> impl IntoView {
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

            match app_context.get_untracked().session.whoami().await {
                Ok(_) => match session_getter.get() {
                    UserSession::Unknown | UserSession::Unauthenticated => {
                        navigate("/auth/login", Default::default())
                    }
                    _ => {}
                },
                Err(e) => {
                    leptos::logging::log!(
                        "AppLayout: app_context.get_untracked().session.whoami().await = {:?}",
                        e
                    );
                    navigate("/auth/login", Default::default())
                }
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
