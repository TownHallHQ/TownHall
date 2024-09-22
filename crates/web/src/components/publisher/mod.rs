use leptos::{component, create_action, create_rw_signal, view, IntoView, SignalGet};
use townhall_client::{post::post_create::post_create::PostCreateInput, Client};

use crate::components::text_field::TextField;

#[component]
pub fn Publisher() -> impl IntoView {
    let text_content = create_rw_signal(String::default());
    let send_post_action = create_action(|content: &String| {
        let content = content.to_owned();

        async move {
            Client::new("http://127.0.0.1:8080")
                .unwrap()
                .post
                .post_create(PostCreateInput {
                    title: content,
                    content: None,
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
            <div id="publisher-form" class="flex flex-col items-start justify-start w-full">
                <form class="flex flex-col justify-start w-full" on:submit=move |ev| {
                            ev.prevent_default();
                            let content = text_content.get();
                            send_post_action.dispatch(content);
                        }>
                    <div class="w-full h-12">
                        <TextField class="w-full" name="content" value=text_content />
                    </div>
                    <div class="flex justify-end items-center">
                        <button type="submit">Post</button>
                    </div>
                </form>
            </div>
        </div>
    }
}
