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
        Gabble is an MIT Licensed solution
        <br />
        Contribute to the project on{" "}
        <a href="https://github.com/whizzes/gabble" class="text-blue-600 underline" target="_blank"> GitHub </a>
      </small>
    </footer>
  </article>
  <small class="text-sm text-white fixed bottom-4 right-4">
    Photo by <a
      class="underline font-semibold"
      href="https://unsplash.com/@{cover?.author?.username}"
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
</main>


<style lang="postcss">
  .auth_view {
    @apply bg-cover bg-center bg-no-repeat flex h-screen overflow-hidden;
  }

  .sidebar {
    @apply bg-white flex flex-col justify-between items-center w-full shadow-md;
  }

  @media screen(md) {
    .sidebar {
      @apply w-[390px];
    }
  }

  .container {
    @apply h-full w-11/12;
  }

  .footer {
    @apply flex flex-col text-gray-600 p-4 text-center;
  }
</style>
