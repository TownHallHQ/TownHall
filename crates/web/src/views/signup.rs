use std::str::FromStr;

use crate::components::text_field::{TextField, TextFieldType};
use leptos::{
    component, create_action, create_rw_signal, create_signal, view, IntoView, Show, SignalGet,
    SignalSet,
};
use townhall_client::Client;
use townhall_types::user::Email;

#[component]
pub fn Signup() -> impl IntoView {
    let (error_getter, error_setter) = create_signal::<Option<String>>(None);

    let name_value = create_rw_signal(String::default());
    let surname_value = create_rw_signal(String::default());
    let username_value = create_rw_signal(String::default());
    let email_value = create_rw_signal(String::default());
    let password_value = create_rw_signal(String::default());

    let submit = create_action(move |_| async move {
        let client = Client::new();
        let res = client
            .auth
            .user_register(townhall_client::auth::user_register::UserRegisterInput {
                name: name_value.get().into(),
                surname: surname_value.get().into(),
                username: username_value.get().into(),
                email: Email::from_str(email_value.get().as_str()).unwrap(),
                password: password_value.get().into(),
            })
            .await;

        if let Some(ref error) = res.error {
            error_setter.set(Some(error.message.to_owned()));
        }
    });

    view! {
      <div class="min-h-screen relative flex justify-center items-center bg-no-repeat bg-cover bg-slate-800 bg-[url('https://images.unsplash.com/photo-1580192985016-7e15ef081dd8?q=80&w=1961&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D')]">
          <div class="absolute bg-black rounded-[50%] w-full h-full blur-[23rem]"></div>
          <div class="flex justify-center items-center z-20">
            <div class="w-full">
              <h1 class="text-6xl text-center font-bold text-white mb-16">TownHall</h1>
              <form class="w-96">
                <TextField class="w-full" name="name" placeholder="Name" value=name_value  />
                <TextField class="w-full" name="surname" placeholder="Surname" value=surname_value/>
                <TextField class="w-full" name="username" placeholder="Username" value=username_value/>
                <TextField class="w-full" name="email" placeholder="Email" value=email_value/>
                <TextField class="w-full" name="password" r#type=TextFieldType::Password placeholder="Password" value=password_value />
                <button type="button" on:click={move |_| submit.dispatch(())} class="bg-blue-700 text-white font-bold w-full mt-3 rounded-md py-3 px-4">Sign up</button>
                <Show when=move ||error_getter.get().is_some()>
                  <div class="bg-rose-600 text-white p-2 rounded-md">
                      {error_getter.get().unwrap()}
                  </div>
                </Show>
              </form>
              <div class="text-center w-full text-white mt-3">{"Do you have an account? "}<a class="underline" href="/login">Log In</a></div>
            </div>
        </div>
      </div>
    }
}
