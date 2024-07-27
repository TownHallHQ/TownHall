use leptos::{component, create_rw_signal, view, IntoView, SignalSet};

use crate::components::{auth::register::SignupCard, icons::home::Home, modal::Modal};

#[component]
pub fn Header() -> impl IntoView {
    let is_open_auth_modal = create_rw_signal(false);
    view! {
            <header class="sticky top-0 bg-white flex items-center justify-center shadow">
                <div class="grid md:grid-cols-12 md:max-w-[1400px] px-4 py-2">
                    <figure class="md:col-start-1"> <img src="https://via.placeholder.com/140x40" />
                    </figure>
                    <nav class="px-4 text-zinc-700 flex justify-start items-center md:col-start-4 md:col-end-10">
                        <a class="flex justify-center items-center h-8 w-8" href="/">
                            <Home class="h-6 w-6" />
                        </a>
                    </nav>
                    <button on:click=move |_| {
                        is_open_auth_modal.set(true);
                      } class="md:col-start-12 md:col-end-13">
                        User
                    </button>
                </div>
            </header>
            <Modal modal_status=is_open_auth_modal>
                <SignupCard />
            </Modal >
    }
}
