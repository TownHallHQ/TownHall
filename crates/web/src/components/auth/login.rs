use leptos::{
    component, create_action, create_rw_signal, create_signal, view, IntoView, RwSignal, Show,
    SignalGet, SignalGetUntracked, SignalSet,
};

use townhall_client::Client;

use crate::components::button::{Button, ButtonType};
use crate::components::text_field::{TextField, TextFieldType};

#[component]
pub fn LoginCard(
    #[prop(into)] signup_status: RwSignal<bool>,
    #[prop(into)] login_status: RwSignal<bool>,
) -> impl IntoView {
    let (error_getter, error_setter) = create_signal::<Option<String>>(None);

    let email_value = create_rw_signal(String::default());
    let password_value = create_rw_signal(String::default());

    let handle_submit = create_action(move |_| async move {
        let client = Client::new("http://localhost:8080");
        let res = client
            .unwrap()
            .auth
            .token_create(email_value.get_untracked(), password_value.get_untracked())
            .await
            .unwrap();

        if let Some(ref error) = res.error {
            error_setter.set(Some(error.message.to_owned()));
        }
    });

    let handle_switch = move |_| {
        login_status.set(false);
        signup_status.set(true);
    };

    view! {

        <div class="w-96 p-6 bg-white border border-gray-200 rounded-lg shadow">
        <h1 class="text-2xl mb-3 text-center">Log in</h1>
              <form class="space-y-2" on:submit=move |_| handle_submit.dispatch(())>
              <TextField
                  class="w-full"
                  name="email"
                  label="Email"
                  placeholder="example@townhall.com"
                  value=email_value
              />
              <TextField
                  class="w-full"
                  label="Password"
                  name="password"
                  r#type=TextFieldType::Password
                  placeholder="* * * * * * *"
                  value=password_value
              />

              <Button r#type=ButtonType::Submit full_width=true>
                  Log in
              </Button>
              <Show when=move || error_getter.get().is_some()>
                  <div class="bg-rose-600 text-white p-2 rounded-md">
                      {error_getter.get().unwrap()}
                  </div>
              </Show>
          </form>
          <div class="text-center mt-3">
              {"Don't have an account? "} <button type="button"  on:click=handle_switch class="underline">
                  Sign up
              </button>
          </div>
              </div>
    }
}
