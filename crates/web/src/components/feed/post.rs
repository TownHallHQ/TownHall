use leptos::{component, view, IntoView};

use townhall_client::post::posts::posts::PostsPostsEdgesNode;

#[component]
pub fn Post(post: PostsPostsEdgesNode) -> impl IntoView {
    view! {
                <li class="feed-post bg-white rounded-md p-4">
                    <div>
                        <article class="flex items-center">
                            <figure class="rounded-md overflow-hidden h-12 w-12">
                                <img src="https://images.unsplash.com/photo-1534528741775-53994a69daeb?q=80&w=3164&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D" alt="A beautiful image" />
                            </figure>
                            <div class="flex flex-col ml-2">
                                <strong>{post.author.username}</strong>
                                <time class="text-sm text-gray-400">2 hours ago</time>
                            </div>
                        </article>
                    </div>
                    <div class="pt-6">
                        <p>
                            {post.title}
                        </p>
                    </div>
                    <div class="flex justify-between items-center pt-6">
                        <button>
                            Share
                        </button>
                        <button>
                            Favorite
                        </button>
                        <button>
                            Comment
                        </button>
                    </div>
                </li>
    }
}
