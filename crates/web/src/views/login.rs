use crate::components::text_field::{TextField, TextFieldType};
use leptos::{component, view, IntoView};

use crate::components::button::{Button, ButtonVariant};

#[component]
pub fn Login() -> impl IntoView {
    view! {
      <div class="min-h-screen flex justify-center items-center bg-no-repeat bg-cover bg-[url('https://images.unsplash.com/photo-1580192985016-7e15ef081dd8?q=80&w=1961&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D')]">
          <div class="flex justify-center items-center">
            <div>
              <h1 class="text-5xl text-center font-bold text-white">TownHall</h1>
              <form class="mt-14">
                <TextField name="email" placeholder="Email" />
                <TextField name="password" r#type=TextFieldType::Password placeholder="Password" />
                <Button variant={ButtonVariant::Contained}>Log in</Button>
              </form>
            </div>
        </div>
      </div>
    }
}
