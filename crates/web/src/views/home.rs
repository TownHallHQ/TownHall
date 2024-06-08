use leptos::{component, view, IntoView};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <section class="flex flex-col justify-center items-center">
            <img src="/assets/img/townhall.png" alt="TownHall AI Generated" height="500" width="500" />
            <h1>{"AI Generated Picture of a TownHall"}</h1>
        </section>
    }
}
