use crate::components::text_field::TextField;
use leptos::{component, view, IntoView};

use crate::components::button::{Button, ButtonVariant};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <section class="flex flex-col justify-center items-center">
            <img src="/assets/img/townhall.png" alt="TownHall AI Generated" height="500" width="500" />
            <h1>{"AI Generated Picture of a TownHall"}</h1>
            <Button variant={ButtonVariant::Text}>{"Text"}</Button>
            <Button variant={ButtonVariant::Contained}>{"Contained"}</Button>
            <Button variant={ButtonVariant::Outlined}>{"Outlined"}</Button>
            <TextField placeholder="Simple" />
            <TextField placeholder="Label" label="Input Label" id="label" />
            <TextField placeholder="Disabled" disabled={true}/>
        </section>
    }
}
