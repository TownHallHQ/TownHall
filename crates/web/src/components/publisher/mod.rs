use leptos::{component, create_rw_signal, view, IntoView};

use crate::components::text_field::TextField;

#[component]
pub fn Publisher() -> impl IntoView {
    let text_content = create_rw_signal(String::default());

    view! {
        <div id="publisher" class="bg-white rounded-md p-4 flex items-start gap-4">
            <div id="publisher-author">
                <figure class="rounded-md overflow-hidden h-12 w-12">
                    <img src="https://images.unsplash.com/photo-1534528741775-53994a69daeb?q=80&w=3164&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D" alt="A beautiful image" />
                </figure>
            </div>
            <div id="publisher-form" class="flex flex-col items-start justify-start w-full">
                <form class="flex flex-col justify-start w-full">
                    <div class="w-full h-12">
                        <TextField class="w-full" name="content" value=text_content />
                    </div>
                    <div class="flex justify-end items-center">
                        <button>Post</button>
                    </div>
                </form>
            </div>
        </div>
    }
}
