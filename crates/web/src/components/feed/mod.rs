mod post;

use leptos::{
    component, create_effect, create_signal, spawn_local, view, For, IntoView, SignalGet, SignalSet,
};

use townhall_client::post::posts::posts::PostsPostsEdges;
use townhall_client::Client;

use self::post::Post;

#[derive(Clone, Debug, PartialEq, Eq)]
enum FeedPosts {
    Loading,
    Ready(Vec<PostsPostsEdges>),
    Error(String),
}

#[component]
pub fn Feed() -> impl IntoView {
    let (posts_getter, posts_setter) = create_signal(FeedPosts::Loading);

    create_effect(move |_| {
        spawn_local(async move {
            match Client::new("http://127.0.0.1:8080")
                .unwrap()
                .post
                .posts(None, None, Some(20), None)
                .await
            {
                Ok(posts) => posts_setter.set(FeedPosts::Ready(posts.edges.to_owned())),
                Err(err) => posts_setter.set(FeedPosts::Error(err.to_string())),
            };
        });
    });

    view! {
        <div id="feed">
            {
                move || match posts_getter.get() {
                    FeedPosts::Loading => view! { <p>Loading...</p> }.into_view(),
                    FeedPosts::Ready(posts) => view! {
                        <ul class="space-y-4">
                            <For
                                each=move || posts.clone()
                                key=|p| p.node.id
                                children=move |PostsPostsEdges {
                                    node,
                                    ..
                                }| {
                                    view! { <Post post=node /> }
                                }
                            />
                        </ul>
                    }.into_view(),
                    FeedPosts::Error(err) => view! { <p>{err}</p> }.into_view(),
                }
            }
        </div>
    }
}
