use leptos::{
    component, create_action, create_rw_signal, view, IntoView, RwSignal, Show, SignalGet,
    SignalSet,
};
use townhall_client::{post::post_create::post_create::PostCreateInput, Client};

use crate::components::text_field::TextField;

#[component]
pub fn Publisher() -> impl IntoView {
    let text_title = create_rw_signal(String::default());
    let text_content = create_rw_signal(String::default());

    let creation_error: RwSignal<Option<String>> = create_rw_signal(None);

    let send_post_action = create_action(move |data: &(String, String)| {
        let (title, content) = data.clone();

        async move {
            let title = title.trim().to_owned();

            if title.is_empty() {
                creation_error.set(Some("Title is required".to_owned()));
                return;
            }

            let content = if content.is_empty() {
                None
            } else {
                Some(content.trim().to_owned())
            };

            let result = Client::new("http://127.0.0.1:8080")
                .unwrap()
                .post
                .post_create(PostCreateInput {
                    title,
                    content,
                    parent_id: None,
                })
                .await;

            if let Err(err) = result {
                creation_error.set(Some(err.to_string()));
            }
        }
    });

    view! {
        <div id="publisher" class="bg-white rounded-md p-4 flex items-start gap-4">
            <div id="publisher-author">
                <figure class="rounded-md overflow-hidden h-12 w-12">
                    <img src="https://images.unsplash.com/photo-1534528741775-53994a69daeb?q=80&w=3164&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D" alt="A beautiful image" />
                </figure>
            </div>
            <div id="publisher-form" class="flex flex-col items-start justify-start w-full space-y-5">
                <form class="flex flex-col justify-start w-full" on:submit=move |ev| {
                            ev.prevent_default();
                            let content = text_content.get();
                            let title = text_title.get();
                            send_post_action.dispatch((title, content));
                        }>
                    <div class="w-full h-12">
                        <TextField class="w-full" name="title" value=text_title placeholder="Title" required=true />

                    </div>
                    <div class="w-full h-12">
                        <TextField class="w-full" name="content" value=text_content placeholder="Content" />
                    </div>
                    <div class="flex justify-end items-center">
                        <button type="submit">Post</button>
                    </div>
                    <Show when=move || creation_error.get().is_some() fallback=move || ()>
                        <p class="text-red-500 mb-3">{creation_error.get().unwrap()}</p>
                    </Show>
                </form>
            </div>
        </div>
    }
}
