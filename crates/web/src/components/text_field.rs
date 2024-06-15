use leptos::{component, view, IntoView};

#[component]
pub fn TextField(
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, into)] value: String,
    #[prop(optional, into, default = "text".to_string())] r#type: String,
) -> impl IntoView {
    view! {
      <>
        <input type=r#type name=name value=value id=id placeholder=placeholder class="p-5 w-full font-semibold text-black bg-black rounded" />
      <>
    }
}
