pub mod components;
pub mod context;
pub mod views;

use leptos::{component, view, IntoView};

use crate::context::AppContext;
use crate::views::View;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <View />
    }
}
