use leptos::{mount_to, view};

use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

use web::components::text_field::{TextField, TextFieldType};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn default_text_field_class_names_integrity() {
    let document = leptos::document();
    let test_wrapper = document.create_element("div").unwrap();
    let _ = document.body().unwrap().append_child(&test_wrapper);

    mount_to(
        test_wrapper.clone().unchecked_into(),
        || view! { <TextField id="text-field" /> },
    );

    let text_field_el = test_wrapper
        .query_selector("#text-field")
        .expect("Failed to access document")
        .expect("text_field Element not found")
        .unchecked_into::<web_sys::HtmlInputElement>();

    let want_class_names = vec![
        "px-3.5",
        "py-2",
        "rounded",
        "font-semibold",
        "placeholder:text-purple-200",
        "border-2",
        "border-purple-300",
        "text-purple-400",
        "focus:border-purple-400",
        "focus:ring-purple-500/60",
    ];

    let have_class_names = text_field_el.get_attribute("class").unwrap();

    for class_name in want_class_names.iter() {
        assert!(have_class_names.contains(class_name));
    }
}

#[wasm_bindgen_test]
fn default_text_field_custom_class_names_integrity() {
    let document = leptos::document();
    let test_wrapper = document.create_element("div").unwrap();
    let _ = document.body().unwrap().append_child(&test_wrapper);

    mount_to(
        test_wrapper.clone().unchecked_into(),
        || view! { <TextField class="w-full" id="text-field" /> },
    );

    let text_field_el = test_wrapper
        .query_selector("#text-field")
        .expect("Failed to access document")
        .expect("text_field Element not found")
        .unchecked_into::<web_sys::HtmlInputElement>();

    let want_class_names = vec![
        "w-full",
        "px-3.5",
        "py-2",
        "rounded",
        "font-semibold",
        "placeholder:text-purple-200",
        "border-2",
        "border-purple-300",
        "text-purple-400",
        "focus:border-purple-400",
        "focus:ring-purple-500/60",
    ];

    let have_class_names = text_field_el.get_attribute("class").unwrap();

    for class_name in want_class_names.iter() {
        assert!(have_class_names.contains(class_name));
    }
}

#[wasm_bindgen_test]
fn text_field_types() {
    assert_eq!(TextFieldType::Text.to_string(), "text");
    assert_eq!(TextFieldType::Email.to_string(), "email");
    assert_eq!(TextFieldType::Password.to_string(), "password");
}
