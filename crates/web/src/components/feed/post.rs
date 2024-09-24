use chrono::{TimeZone, Utc};
use leptos::{component, view, IntoView};

use townhall_client::post::posts::posts::PostsPostsEdgesNode;

const LONG_TO_SHORT_CUTOFF_DAYS: i64 = 3;

#[component]
pub fn Post(post: PostsPostsEdgesNode) -> impl IntoView {
    let post_created_at_date_time = Utc::from_utc_datetime(&Utc, &post.created_at.naive_utc());
    let now_date = Utc::now();

    let diff_in_seconds = now_date
        .signed_duration_since(post_created_at_date_time)
        .num_seconds();

    let format_string = match diff_in_seconds {
        _ if diff_in_seconds > 86400 * LONG_TO_SHORT_CUTOFF_DAYS => "%m/%d/%y".to_string(), // More than 3 days
        diff_in_seconds if diff_in_seconds > 86400 => {
            format!("{} Days ago", diff_in_seconds / 86400)
        } // Within 3 days
        diff_in_seconds if diff_in_seconds > 3600 => {
            format!("{} hours ago", diff_in_seconds / 3600)
        } // Within 1 day
        diff_in_seconds if diff_in_seconds > 60 => {
            format!("{} minutes ago", diff_in_seconds / 60)
        } // Within 1 hour
        _ => format!("{} seconds ago", diff_in_seconds),
    };

    let formatted_date = post_created_at_date_time.format(&format_string).to_string();

    view! {
                <li class="feed-post bg-white rounded-md p-4">
                    <div>
                        <article class="flex items-center">
                            <figure class="rounded-md overflow-hidden h-12 w-12">
                                <img src="https://images.unsplash.com/photo-1534528741775-53994a69daeb?q=80&w=3164&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D" alt="A beautiful image" />
                            </figure>
                            <div class="flex flex-col ml-2">
                                <strong>{post.author.username}</strong>
                                <time class="text-sm text-gray-400">{formatted_date}</time>
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
