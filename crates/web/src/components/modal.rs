use leptos::{
    component, create_node_ref, html::Div, store_value, view, ChildrenFn, IntoView, RwSignal, Show,
    SignalGet, SignalSet, Suspense,
};
use leptos_use::on_click_outside;

#[component]
pub fn Modal(#[prop(into)] modal_status: RwSignal<bool>, children: ChildrenFn) -> impl IntoView {
    let modal_ref = create_node_ref::<Div>();
    let children = store_value(children);

    on_click_outside(modal_ref, move |_event| modal_status.set(false));

    view! {
        <Suspense fallback=|| ()>
            <Show when=move || modal_status.get() fallback=move || ()>
                <div class="fixed right-0 top-0 z-10 flex h-screen w-screen flex-col items-center justify-center drop-shadow-lg backdrop-blur-[2px] bg-[#00000055]">
                    <div node_ref=modal_ref>{children.with_value(|children| children())}</div>
                </div>
            </Show>
        </Suspense>
    }
}
