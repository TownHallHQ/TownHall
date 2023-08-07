<script lang="ts">
  import { onMount } from 'svelte';

  import intersectionObserver from '$lib/actions/intersection-observer';
  import Post from './Post.svelte';

  import type { Post as PostType } from '$lib/graphql/schema';

  let pageNumber = 10;
  let posts: PostType[] = [];
  let loading = false;
  let lastPostId: string = '';

  onMount(() => {
    fetchData();
  });

  async function fetchData() {
    try {
      loading = true;
      const response = await fetch(
        `/get-posts?first=${pageNumber}&after=${lastPostId}`
      );
      const data: PostType[] = await response.json();

      posts = [...posts, ...data];

      lastPostId = data.pop()?.id;

      console.log(posts);

      loading = false;
    } catch (error) {
      console.log(error);
    }
  }

  const load = () => {
    pageNumber = pageNumber + 10;
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
    <div class="mb-96">Cargando...</div>
  {/if}
  {#if loading === false}
    <div class="mb-96" bind:this={elementRef}>Cargando</div>
  {/if}
</div>
