use leptos::{component, view, IntoView};

use crate::components::feed::Feed;
use crate::components::publisher::Publisher;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="space-y-4 pt-4 md:col-start-4 md:col-end-10">
            <Publisher />
            <Feed />
        </div>
    }
}
