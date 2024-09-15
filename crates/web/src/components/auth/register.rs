use std::str::FromStr;

use leptos::{
    component, create_action, create_rw_signal, create_signal, view, IntoView, RwSignal, Show,
    SignalGet, SignalGetUntracked, SignalSet,
};

use townhall_client::Client;
use townhall_types::user::Email;

use crate::components::button::{Button, ButtonType};
use crate::components::text_field::{TextField, TextFieldType};

#[component]
pub fn SignupCard(
    #[prop(into)] signup_status: RwSignal<bool>,
    #[prop(into)] login_status: RwSignal<bool>,
) -> impl IntoView {
    let (error_getter, error_setter) = create_signal::<Option<String>>(None);

    let name_value = create_rw_signal(String::default());
    let surname_value = create_rw_signal(String::default());
    let username_value = create_rw_signal(String::default());
    let email_value = create_rw_signal(String::default());
    let password_value = create_rw_signal(String::default());

    let submit = create_action(move |_| async move {
        let client = Client::new("http://localhost:8080");
        let res = client
            .unwrap()
            .auth
            .user_register(townhall_client::auth::user_register::UserRegisterInput {
                name: name_value.get_untracked(),
                surname: surname_value.get_untracked(),
                username: username_value.get_untracked(),
                email: Email::from_str(email_value.get_untracked().as_str()).unwrap(),
                password: password_value.get_untracked(),
            })
            .await
            .unwrap();

        if let Some(ref error) = res.error {
            error_setter.set(Some(error.message.to_owned()));
        }
    });

    let handle_switch = move |_| {
        signup_status.set(false);
        login_status.set(true);
    };

    view! {

        <div class="w-96 p-6 bg-white border border-gray-200 rounded-lg shadow">
        <h1 class="text-2xl mb-3 text-center">Sign up</h1>
              <form class="space-y-2">
              <div class="flex justify-between space-x-2">
              <TextField class="w-full" name="name" placeholder="Name" label="Name" value=name_value />
              <TextField
                class="w-full"
                label="Surname"
                name="surname"
                placeholder="Surname"
                value=surname_value
              />
              </div>
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
                  r#type=TextFieldType::Email
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
              <Button
              r#type=ButtonType::Button
              on:click=move |_| submit.dispatch(())
        full_width=true
              >
                  Sign up
              </Button>
              <Show when=move || error_getter.get().is_some()>
                  <div class="bg-rose-600 text-white p-2 rounded-md">
                      {error_getter.get().unwrap()}
                  </div>
              </Show>
          </form>
          <div class="text-center mt-3">
              {"Do you have an account? "} <button on:click=handle_switch class="underline" >
                  Log In
              </button>
          </div>
              </div>


    }
}
