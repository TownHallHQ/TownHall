mod post;

use leptos::{component, view, IntoView};

use self::post::Post;

#[component]
pub fn Feed() -> impl IntoView {
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
