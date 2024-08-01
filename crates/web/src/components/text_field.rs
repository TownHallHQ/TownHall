use core::fmt;
use std::{collections::HashSet, fmt::Debug};

use leptos::{
    component, create_memo, event_target_value, view, IntoView, MaybeProp, RwSignal, Show,
    SignalGet, SignalSet, TextProp,
};

#[derive(Clone, Debug, Default)]
pub enum TextFieldVariant {
    #[default]
    Primary,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum TextFieldType {
    #[default]
    Text,
    Email,
    Password,
}

impl fmt::Display for TextFieldType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TextFieldType::Text => write!(f, "text"),
            TextFieldType::Email => write!(f, "email"),
            TextFieldType::Password => write!(f, "password"),
        }
    }
}

#[component]
pub fn TextField(
    #[prop(into)] value: RwSignal<String>,
    #[prop(optional, into)] name: TextProp,
    #[prop(optional, into)] id: MaybeProp<TextProp>,
    #[prop(optional, into)] placeholder: TextProp,
    #[prop(optional, into)] label: MaybeProp<TextProp>,
    #[prop(optional, into)] class: TextProp,
    #[prop(optional, into)] variant: MaybeProp<TextFieldVariant>,
    #[prop(optional, into)] r#type: TextFieldType,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] full_width: MaybeProp<bool>,
) -> impl IntoView {
    let label_cloned = label.clone();
    let custom_classes = class.get();
    let class_names = create_memo(move |_| {
        let mut classes: HashSet<&str> = HashSet::new();

        classes.insert(custom_classes.as_str());

        match variant.get().unwrap_or_default() {
            TextFieldVariant::Primary => {
                classes.insert("bg-gray-100");
                classes.insert("rounded-md");
                classes.insert("placeholder:text-purple-200");
                classes.insert("border");
                classes.insert("border-gray-100");
                classes.insert("px-2");
                classes.insert("py-1");
                classes.insert("text-md");
                classes.insert("focus:border-indigo-400");
                classes.insert("focus:ring-indigo-500/60");
            }
        }

        // Default Classes
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

    let handle_change = move |ev| {
        value.set(event_target_value(&ev));
    };

    let handle_input = move |ev| {
        value.set(event_target_value(&ev));
    };

    view! {
        <div>
            <Show when=move || label_cloned.get().is_some() fallback=move || ()>
                <label class="block mb-2 text-sm font-medium text-gray-900">
                    {label.get()}
                </label>
            </Show>
            <input
                type=format!("{}", r#type)
                name=name
                value=value
                id=id
                placeholder=placeholder
                class=class_names
                disabled=disabled
                on:change=handle_change
                on:input=handle_input
            />
        </div>
    }
}
