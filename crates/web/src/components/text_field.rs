use std::collections::HashSet;

use leptos::{component, create_memo, view, IntoView, MaybeProp, SignalGet};

#[component]
pub fn TextField(
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, into)] value: String,
    #[prop(optional, into)] label: String,
    #[prop(optional, into, default = "text".to_string())] r#type: String,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] full_width: MaybeProp<bool>,
) -> impl IntoView {
    println!("label {}", label);
    let class_names = create_memo(move |_| {
        let mut classes: HashSet<&str> = HashSet::new();

        // Default Classes
        classes.insert("px-3.5");
        classes.insert("py-2");
        classes.insert("rounded");
        classes.insert("font-semibold");
        classes.insert("placeholder:text-purple-200");
        classes.insert("border-2");
        classes.insert("border-purple-300");
        classes.insert("text-purple-400");
        classes.insert("focus:border-purple-400");
        classes.insert("focus:ring-purple-500/60");

        if let Some(is_full_width) = full_width.get() {
            if is_full_width {
                classes.insert("w-full");
            }
        }

        if let Some(is_disabled) = disabled.get() {
            if is_disabled {
                classes.insert("opacity-70");
                classes.insert("!cursor-not-allowed");
            }
        }

        classes.into_iter().collect::<Vec<&str>>().join(" ")
    });
    view! {
      <div>
        <label class="block mb-2 text-sm font-medium text-purple-500" for=&id>{label}</label>
        <input type=r#type name=name value=value id=id placeholder=placeholder class=class_names  disabled=disabled />
      </div>
    }
}
