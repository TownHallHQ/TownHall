use leptos::{component, view, IntoView};
use leptos_router::Outlet;

#[component]
pub fn AuthLayout() -> impl IntoView {
    view! {
        <div>
            <main>
                <Outlet />
            </main>
        </div>
    }
}
