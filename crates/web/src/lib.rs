pub mod components;
pub mod context;
pub mod views;

use leptos::{component, create_rw_signal, provide_context, view, IntoView};
use leptos_meta::provide_meta_context;

use crate::context::Context;
use crate::views::View;

#[component]
pub fn App() -> impl IntoView {
    provide_context(create_rw_signal(Context::default()));
    provide_meta_context();

    view! {
        <View />
    }
}
