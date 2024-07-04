use leptos::{mount_to, view};

use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

use web::components::button::{Button, ButtonVariant};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn default_text_button_class_names_integrity() {
    let document = leptos::document();
    let test_wrapper = document.create_element("div").unwrap();
    let _ = document.body().unwrap().append_child(&test_wrapper);

    mount_to(
        test_wrapper.clone().unchecked_into(),
        || view! { <Button id="btn-text" variant={ButtonVariant::Text}>"Click Me!"</Button> },
    );

    let button_el = test_wrapper
        .query_selector("#btn-text")
        .expect("Failed to access document")
        .expect("Button Element not found")
        .unchecked_into::<web_sys::HtmlButtonElement>();

    let want_class_names = [
        "px-4",
        "py-2",
        "rounded-md",
        "cursor-pointer",
        "font-semibold",
        "text-sm",
        "btn-text",
        "bg-transparent",
    ];

    let have_class_names = button_el.get_attribute("class").unwrap();

    for class_name in want_class_names.iter() {
        assert!(have_class_names.contains(class_name));
    }
}

#[wasm_bindgen_test]
fn default_text_button_custom_class_names_integrity() {
    let document = leptos::document();
    let test_wrapper = document.create_element("div").unwrap();
    let _ = document.body().unwrap().append_child(&test_wrapper);

    mount_to(
        test_wrapper.clone().unchecked_into(),
        || view! { <Button id="btn-text" class="custom-class" variant={ButtonVariant::Text}>"Click Me!"</Button> },
    );

    let button_el = test_wrapper
        .query_selector("#btn-text")
        .expect("Failed to access document")
        .expect("Button Element not found")
        .unchecked_into::<web_sys::HtmlButtonElement>();

    let want_class_names = [
        "custom-class",
        "px-4",
        "py-2",
        "rounded-md",
        "cursor-pointer",
        "font-semibold",
        "text-sm",
        "btn-text",
        "bg-transparent",
    ];

    let have_class_names = button_el.get_attribute("class").unwrap();

    for class_name in want_class_names.iter() {
        assert!(have_class_names.contains(class_name));
    }
}
