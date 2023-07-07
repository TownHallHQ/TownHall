<script lang="ts">
  import { onMount } from 'svelte';
  import Card from '$lib/components/Card.svelte';
  import type { Unsplash } from '../api/unsplash/+server';

  let cover: Unsplash;

  onMount(async () => {
    const response = await fetch('/api/unsplash');
    const data = await response.json();
    cover = data.data;

    document.documentElement.style.setProperty(
      '--cover-image',
      `url('${cover?.url}')`
    );
  });
</script>

<div
  class="flex bg-gray-900 min-h-screen justify-center items-center bg-cover bg-no-repeat"
  style="background-image: var(--cover-image);"
>
  <Card class="h-max w-full mx-2 md:w-1/2 xl:w-1/4">
    <slot />
  </Card>
  <small class="text-sm text-white fixed bottom-4 right-4"
    >Photo by <a
      class="underline font-semibold"
      href={`https://unsplash.com/@${cover?.author?.username}?utm_source=gabble&utm_medium=referral`}
      >{cover?.author?.name || ''}</a
    >
    on
    <a
      class="underline font-medium"
      href="https://unsplash.com/?utm_source=gabble&utm_medium=referral"
    >
      Unsplash</a
    ></small
  >
</div>
