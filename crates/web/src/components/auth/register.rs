use std::str::FromStr;

use leptos::{
    component, create_action, create_rw_signal, create_signal, view, IntoView, Show, SignalGet,
    SignalGetUntracked, SignalSet,
};

use townhall_client::Client;
use townhall_types::user::Email;

use crate::components::text_field::{TextField, TextFieldType};

#[component]
pub fn SignupCard() -> impl IntoView {
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
                name: name_value.get_untracked().into(),
                surname: surname_value.get_untracked().into(),
                username: username_value.get_untracked().into(),
                email: Email::from_str(email_value.get_untracked().as_str()).unwrap(),
                password: password_value.get_untracked().into(),
            })
            .await;

        if let Some(ref error) = res.error {
            error_setter.set(Some(error.message.to_owned()));
        }
    });

    view! {

        <div class="w-96 p-6 bg-white border border-gray-200 rounded-lg shadow">
        <h1 class="text-2xl mb-3 text-center">Sign up</h1>
              <form class="space-y-2">
              <TextField class="w-full" name="name" placeholder="Name" value=name_value/>
              <TextField
                  class="w-full"
                  label="Surname"
                  name="surname"
                  placeholder="Surname"
                  value=surname_value
              />
              <TextField
                  class="w-full"
                  label="Username"
                  name="username"
                  placeholder="Username"
                  value=username_value
              />
              <TextField
                  class="w-full"
                  name="email"
                  label="Email"
                  placeholder="Email"
                  value=email_value
              />
              <TextField
                  class="w-full"
                  label="Password"
                  name="password"
                  r#type=TextFieldType::Password
                  placeholder="Password"
                  value=password_value
              />
              <button
                  type="button"
                  on:click=move |_| submit.dispatch(())
                  class="bg-blue-700 text-white font-bold w-full mt-3 rounded-md py-3 px-4"
              >
                  Sign up
              </button>
              <Show when=move || error_getter.get().is_some()>
                  <div class="bg-rose-600 text-white p-2 rounded-md">
                      {error_getter.get().unwrap()}
                  </div>
              </Show>
          </form>
          <div class="text-center mt-3">
              {"Do you have an account? "} <a class="underline" href="/login">
                  Log In
              </a>
          </div>
              </div>


    }
}
