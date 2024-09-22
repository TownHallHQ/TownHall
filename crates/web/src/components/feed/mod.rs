mod post;

use leptos::{component, create_resource, view, IntoView};
use townhall_client::Client;

use self::post::Post;

#[component]
pub fn Feed() -> impl IntoView {
    let posts = create_resource(
        || (),
        |_| async move {
            let client = Client::new("http://localhost:8080");

            let posts = client
                .unwrap()
                .post
                .posts(None, None, Some(10), None)
                .await
                .unwrap();

            posts
        },
    );

    view! {
        <div id="feed">
            <ul class="space-y-4">
                <Post />
                <Post />
                <Post />
                <Post />
            </ul>
        </div>
    }
}
