use leptos::{component, create_action, create_rw_signal, view, IntoView, SignalGet};
use townhall_client::{post::post_create::post_create::PostCreateInput, Client};

use crate::components::text_field::TextField;

#[component]
pub fn Publisher() -> impl IntoView {
    let text_title = create_rw_signal(String::default());
    let text_content = create_rw_signal(String::default());

    let send_post_action = create_action(|data: &(String, String)| {
        let (title, content) = data;

        let title = title.to_owned();

        if title.is_empty() {
            ()
        }

        let content = if content.trim().is_empty() {
            None
        } else {
            Some(content.to_owned())
        };

        async move {
            Client::new("http://127.0.0.1:8080")
                .unwrap()
                .post
                .post_create(PostCreateInput {
                    title: title,
                    content: content,
                    parent_id: None,
                })
                .await
        }
    });

    view! {
        <div id="publisher" class="bg-white rounded-md p-4 flex items-start gap-4">
            <div id="publisher-author">
                <figure class="rounded-md overflow-hidden h-12 w-12">
                    <img src="https://images.unsplash.com/photo-1534528741775-53994a69daeb?q=80&w=3164&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D" alt="A beautiful image" />
                </figure>
            </div>
            <div id="publisher-form" class="flex flex-col items-start justify-start w-full space-y-2">
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
                </form>
            </div>
        </div>
    }
}
