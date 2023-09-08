<script lang="ts">
  import { onMount } from 'svelte';
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

<main class="auth_view" style="background-image: url({cover?.url});">
  <article class="sidebar">
    <div class="container">
      <slot />
    </div>
    <footer class="footer">
      <small>
        playa is an MIT Licensed solution
        <br />
        Contribute to the project on{" "}
        <a href="https://github.com/whizzes/playa" class="text-blue-600 underline" target="_blank"> GitHub </a>
      </small>
    </footer>
  </article>
  <small class="hidden md:inline text-sm text-white fixed bottom-4 right-4">
    Photo by <a
      class="underline font-semibold"
      href="https://unsplash.com/@{cover?.author?.username}"
      >{cover?.author?.name || ''}</a
    >
    on
    <a
      class="underline font-medium"
      href="https://unsplash.com/?utm_source=playa&utm_medium=referral"
    >
      Unsplash</a
    ></small
  >
</main>


<style lang="postcss">
  .auth_view {
    @apply bg-cover bg-center bg-no-repeat flex h-screen overflow-hidden;
    @apply grid grid-cols-4;
  }

  .sidebar {
    @apply bg-white flex flex-col justify-between items-center w-full shadow-md;
    @apply col-span-4;
  }

  .container {
    @apply h-full w-full p-6 bg-neutral-50 dark:bg-neutral-900;
  }

  .footer {
    @apply flex flex-col text-gray-600 p-4 text-center p-6 bg-neutral-50 dark:bg-neutral-900 w-full;
  }

  @media screen(md) {
    .auth_view {
      @apply grid-cols-12 col-span-12;
    }

    .sidebar {
      @apply col-span-4;
    }
  }
</style>
