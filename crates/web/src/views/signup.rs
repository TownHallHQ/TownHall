use leptos::{component, view, IntoView};

use crate::components::text_field::{TextField, TextFieldType};

#[component]
pub fn Signup() -> impl IntoView {
    view! {
      <div class="min-h-screen relative flex justify-center items-center bg-no-repeat bg-cover bg-slate-800 bg-[url('https://images.unsplash.com/photo-1580192985016-7e15ef081dd8?q=80&w=1961&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D')]">
          <div class="absolute bg-black rounded-[50%] w-full h-full blur-[23rem]"></div>
          <div class="flex justify-center items-center z-20">
            <div class="w-full">
              <h1 class="text-6xl text-center font-bold text-white mb-16">TownHall</h1>
              <form class="w-96">
              <TextField class="w-full" name="name" placeholder="Name" />
              <TextField class="w-full" name="surname" placeholder="Surname" />
              <TextField class="w-full" name="username" placeholder="Username" />
                <TextField class="w-full" name="email" placeholder="Email" />
                <TextField class="w-full" name="password" r#type=TextFieldType::Password placeholder="Password" />
                <button type="button" class="bg-blue-700 text-white font-bold w-full mt-3 rounded-md py-3 px-4">Login</button>

              </form>
              <div class="text-center w-full text-white mt-3">{"Do you have an account? "}<a class="underline" href="/login">Log In</a></div>
            </div>
        </div>
      </div>
    }
}
