use leptos::{component, create_action, create_signal, view, IntoView, Show, SignalGet, SignalSet};

use townhall_client::auth::token_create::token_create;

use crate::components::text_field::{TextField, TextFieldType};

#[component]
pub fn Login() -> impl IntoView {
    let (error_getter, error_setter) = create_signal::<Option<String>>(None);

    let submit = create_action(move |_| async move {
        let res = token_create("john@test.com".into(), "12345".into()).await;

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
                <TextField class="w-full" name="email" placeholder="Email" />
                <TextField class="w-full" name="password" r#type=TextFieldType::Password placeholder="Password" />
                <button type="button" on:click={move |_| submit.dispatch(())}>Login</button>
                <Show when=move ||error_getter.get().is_some()>
                    <div class="bg-rose-600 text-white p-2 rounded-md">
                        {error_getter.get().unwrap()}
                    </div>
                </Show>
              </form>
              <div class="text-center w-full text-white mt-3">{"Don't have an account? "}<a class="underline" href="/signup">Sign up!</a></div>
            </div>
        </div>
      </div>
    }
}
