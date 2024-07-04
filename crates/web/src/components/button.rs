use std::collections::HashSet;

use leptos::{component, create_memo, view, Children, IntoView, MaybeProp, SignalGet, TextProp};

use super::helper::maybe_children::MaybeChildren;

#[derive(Clone, Debug, Default)]
pub enum ButtonVariant {
    Text,
    #[default]
    Contained,
    Outlined,
}

#[component]
pub fn Button(
    #[prop(optional, into)] id: TextProp,
    #[prop(optional, into)] class: TextProp,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] variant: MaybeProp<ButtonVariant>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let custom_classes = class.get();
    let class_names = create_memo(move |_| {
        let mut classes: HashSet<&str> = HashSet::new();

        classes.insert(custom_classes.as_str());

        // Default Classes
        classes.insert("px-4");
        classes.insert("py-2");
        classes.insert("rounded-md");
        classes.insert("cursor-pointer");
        classes.insert("font-semibold");
        classes.insert("text-sm");

        if let Some(is_disabled) = disabled.get() {
            if is_disabled {
                classes.insert("opacity-70");
                classes.insert("!cursor-not-allowed");
            }
        }

        match variant.get().unwrap_or_default() {
            ButtonVariant::Text => {
                classes.insert("btn-text");
                classes.insert("bg-transparent");
            }
            ButtonVariant::Contained => {
                classes.insert("btn-contained");
                classes.insert("bg-purple-500");
                classes.insert("text-white");
            }
            ButtonVariant::Outlined => {
                classes.insert("btn-outlined");
                classes.insert("bg-transparent");
                classes.insert("border-2");
                classes.insert("border-purple-500");
            }
        }

        classes.into_iter().collect::<Vec<&str>>().join(" ")
    });

    view! {
        <button id={id} class={class_names} disabled=move || disabled.get()>
            <MaybeChildren value=children let:children>
                {children()}
            </MaybeChildren>
        </button>
    }
}
