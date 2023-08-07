<script lang="ts">
  import { onMount } from 'svelte';

  import intersectionObserver from '$lib/actions/intersection-observer';
  import Post from './Post.svelte';

  import type { Post as PostType } from '$lib/graphql/schema';

  let posts: PostType[] = [];
  let loading = false;
  let lastPostId: string = '';
  let noMorePosts: boolean = false;

  onMount(() => {
    fetchData();
  });

  async function fetchData() {
    try {
      loading = true;
      const response = await fetch(`/get-posts?first=20&after=${lastPostId}`);
      const data: PostType[] = await response.json();

      if (data.length > 0) {
        posts = [...posts, ...data];

        lastPostId = data[data.length - 1].id;
      } else {
        noMorePosts = true;
      }
      loading = false;
    } catch (error) {
      console.log(error);
    }
  }

  const load = () => {
    fetchData();
  };

  let elementRef: Element;
  $: {
    if (elementRef) {
      intersectionObserver({ fetch: load, element: elementRef });
    }
  }
</script>

{#each posts as post}
  <Post {post} />
{/each}

<div class="mb-96">
  {#if loading}
    <span class="mb-20">Loading...</span>
  {/if}
  {#if loading === false && !noMorePosts}
    <span class="mb-20" bind:this={elementRef}>Loading</span>
  {/if}
  {#if noMorePosts}
    <span>No more posts</span>
  {/if}
</div>
