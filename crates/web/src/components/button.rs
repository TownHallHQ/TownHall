use core::fmt;
use std::collections::HashSet;
use std::rc::Rc;

use leptos::web_sys::MouseEvent;
use leptos::{component, create_memo, view, Children, IntoView, MaybeProp, SignalGet, TextProp};

use super::helper::maybe_children::MaybeChildren;

#[derive(Clone, Debug, Default)]
pub enum ButtonVariant {
    Text,
    #[default]
    Contained,
    Outlined,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ButtonType {
    #[default]
    Button,
    Submit,
}

impl fmt::Display for ButtonType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ButtonType::Submit => write!(f, "submit"),
            ButtonType::Button => write!(f, "button"),
        }
    }
}

#[component]
pub fn Button(
    #[prop(optional, into)] id: TextProp,
    #[prop(optional, into)] class: TextProp,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional, into)] full_width: MaybeProp<bool>,
    #[prop(optional, into)] variant: MaybeProp<ButtonVariant>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional, into)] on_click: MaybeProp<Rc<dyn Fn(MouseEvent) -> () + 'static>>,
    #[prop(optional, into)] r#type: ButtonType,
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

        match variant.get().unwrap_or_default() {
            ButtonVariant::Text => {
                classes.insert("btn-text");
                classes.insert("bg-transparent");
            }
            ButtonVariant::Contained => {
                classes.insert("btn-contained");
                classes.insert("bg-blue-700");
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

    let handle_click = move |event| {
        if let Some(on_click) = on_click.get() {
            on_click.clone()(event);
        }
    };

    view! {
        <button
            type=format!("{}", r#type)
            id=id
            class=class_names
            disabled=move || disabled.get()
            on:click=handle_click
        >
            <MaybeChildren value=children let:children>
                {children()}
            </MaybeChildren>
        </button>
    }
}
