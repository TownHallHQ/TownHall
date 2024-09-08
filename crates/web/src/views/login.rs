use leptos::{
    component, create_action, create_rw_signal, create_signal, view, IntoView, Show, SignalGet,
    SignalGetUntracked,
};

use townhall_client::Client;

use crate::components::text_field::{TextField, TextFieldType};

#[component]
pub fn Login() -> impl IntoView {
    let (error_getter, error_setter) = create_signal::<Option<String>>(None);
    let email_value = create_rw_signal(String::default());
    let password_value = create_rw_signal(String::default());
    let handle_submit = create_action(move |_| async move {
        let client = Client::new("http://127.0.0.1:8080").unwrap();
        let res = client
            .auth
            .login(email_value.get_untracked(), password_value.get_untracked())
            .await;
    });

    view! {
        <div class="min-h-screen relative flex justify-center items-center bg-no-repeat bg-cover bg-slate-800 bg-[url('https://images.unsplash.com/photo-1580192985016-7e15ef081dd8?q=80&w=1961&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D')]">
            <div class="absolute bg-black rounded-[50%] w-full h-full blur-[23rem]"></div>
            <div class="flex justify-center items-center z-20">
                <div class="w-full">
                    <h1 class="text-6xl text-center font-bold text-white mb-16">TownHall</h1>
                    <form class="w-96" on:submit=move |ev| {
                        ev.prevent_default();
                        handle_submit.dispatch(());
                    }>
                        <TextField
                            class="w-full"
                            name="email"
                            placeholder="Email"
                            value=email_value
                        />
                        <TextField
                            class="w-full"
                            name="password"
                            r#type=TextFieldType::Password
                            placeholder="Password"
                            value=password_value
                        />
                        <button type="submit">Login</button>
                        <Show when=move || error_getter.get().is_some()>
                            <div class="bg-rose-600 text-white p-2 rounded-md">
                                {error_getter.get().unwrap()}
                            </div>
                        </Show>
                    </form>
                    <div class="text-center w-full text-white mt-3">
                        {"Don't have an account? "} <a class="underline" href="/auth/signup">
                            Sign up!
                        </a>
                    </div>
                </div>
            </div>
        </div>
    }
}
