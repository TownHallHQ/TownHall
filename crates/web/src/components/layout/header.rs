use leptos::{component, view, IntoView};

use crate::components::icons::home::Home;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="sticky top-0 bg-white flex items-center justify-center shadow">
            <div class="grid md:grid-cols-12 md:max-w-[1400px] px-4 py-2">
                <figure class="md:col-start-1">
                    <img src="https://via.placeholder.com/140x40" />
                </figure>
                <nav class="px-4 text-zinc-700 flex justify-start items-center md:col-start-4 md:col-end-10">
                    <a class="flex justify-center items-center h-8 w-8" href="/">
                        <Home class="h-6 w-6" />
                    </a>
                </nav>
                <div class="md:col-start-12 md:col-end-13">
                    User
                </div>
            </div>
        </header>
    }
}
