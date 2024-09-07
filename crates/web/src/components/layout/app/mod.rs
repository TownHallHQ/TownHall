mod header;

use leptos::{component, view, IntoView};
use leptos_router::Outlet;

use self::header::Header;

#[component]
pub fn AppLayout() -> impl IntoView {
    view! {
        <div class="bg-gray-50 min-h-screen">
            <Header />
            <main class="grid md:grid-cols-12 md:max-w-[1400px] px-4 py-2 mx-auto">
                <Outlet />
            </main>
        </div>
    }
}
