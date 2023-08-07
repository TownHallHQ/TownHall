<script lang="ts">
  import { page } from '$app/stores';
  import intersectionObserver from '$lib/actions/intersection-observer';
  import Post from './Post.svelte';

  let pageNumber = 0;
  let posts = $page.data.posts;
  let Loading = false;

  async function fetchData() {
    try {
      Loading = true;
      const response = await fetch(`/get-posts?page=${pageNumber}`, {
        method: 'GET',
      });
      const data = await response.json();

      posts = [...(posts || []), ...data];

      Loading = false;
    } catch (error) {
      console.log(error);
    }
  }

  const load = () => {
    pageNumber = pageNumber + 1;
    fetchData();
  };

  let elementRef: Element;
  $: {
    if (elementRef) {
      intersectionObserver({ fetch: load, element: elementRef });
    }
  }
</script>

{#each posts || [] as { node: post }}
  <Post {post} />
{/each}

<div class="mb-96">
  {#if Loading}
    <div class="mb-96">Cargando...</div>
  {/if}
  <!-- ELEMENT OBSERVER -->
  {#if Loading === false}
    <div class="mb-96" bind:this={elementRef}>Cargando</div>
  {/if}
</div>
