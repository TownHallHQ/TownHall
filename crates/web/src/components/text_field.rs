use core::fmt;
use std::{collections::HashSet, fmt::Debug};

use leptos::{component, create_memo, logging, view, IntoView, MaybeProp, SignalGet, TextProp};

#[derive(Clone, Debug, Default)]
pub enum TextFieldVariant {
    #[default]
    Primary,
}

#[derive(Clone, Debug, Default)]
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
    #[prop(optional, into)] name: TextProp,
    #[prop(optional, into)] id: TextProp,
    #[prop(optional, into)] placeholder: TextProp,
    #[prop(optional, into)] value: TextProp,
    #[prop(optional, into)] label: TextProp,
    #[prop(optional, into)] variant: MaybeProp<TextFieldVariant>,
    #[prop(optional, into)] r#type: TextFieldType,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] full_width: MaybeProp<bool>,
) -> impl IntoView {
    let class_names = create_memo(move |_| {
        let mut classes: HashSet<&str> = HashSet::new();

        match variant.get().unwrap_or_default() {
            TextFieldVariant::Primary => {
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

    logging::log!("{}", r#type);

    view! {
            <div>
            <label class="block mb-2 text-sm font-medium text-purple-500" for=id.clone()>{label}</label>
            <input type=format!("{}", r#type) name=name value=value id=id placeholder=placeholder class=class_names  disabled=disabled />
            </div>
    }
}
