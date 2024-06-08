use leptos::{mount_to_body, view};

use web::App;

fn main() {
    mount_to_body(|| {
        view! { <App/> }
    })
}
