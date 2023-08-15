<script lang="ts">
  import { onMount } from "svelte";
  import Placeholder from "@whizzes/svelte-placeholder";

  import intersectionObserver from "$lib/actions/intersection-observer";
  import Card from "$lib/components/Card.svelte";
  import Post from "./Post.svelte";

  import type { PostConnection, Post as PostType } from "$lib/graphql/schema";

  let className: string | null = null;
  export { className as class };

  export let username: string = "";

  let posts: PostType[] = [];
  let loadingPosts = false;
  let lastPostId: string = "";
  let noMorePosts: boolean = false;

  onMount(() => {
    fetchData();
  });

  async function fetchData() {
    try {
      loadingPosts = true;

      const response = await fetch(
        `/get-posts?first=20&after=${lastPostId}&username=${username}`
      );
      const data: PostConnection = await response.json();

      if (data.nodes.length > 0) {
        posts = [...posts, ...data.nodes];

        lastPostId = data.pageInfo.endCursor || "";
      } else {
        noMorePosts = true;
      }
      loadingPosts = false;
    } catch (error) {
      loadingPosts = false;
      console.log(error);
    }
  }

  let elementRef: Element;
  $: {
    if (elementRef) {
      intersectionObserver({ fetch: fetchData, element: elementRef });
    }
  }
</script>

<div class="space-y-5 {className}">
  {#each posts as post}
    <Post {post} />
  {/each}

  {#if loadingPosts}
    {#each Array.from({ length: 6 }) as _}
      <div>
        <Card class="h-[160px]">
          <Placeholder>
            <rect x="2" y="2" rx="8" ry="8" width="63" height="62" />
            <rect x="75" y="3" rx="8" ry="8" width="149" height="22" />
            <rect x="77" y="35" rx="8" ry="8" width="110" height="19" />
            <rect x="2" y="72" rx="8" ry="8" width="130" height="25" />
            <rect x="2" y="110" rx="8" ry="8" width="394" height="19" />
          </Placeholder>
        </Card>
      </div>
    {/each}
  {/if}
</div>
