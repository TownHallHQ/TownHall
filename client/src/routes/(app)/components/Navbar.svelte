<script lang="ts">
  import { page } from '$app/stores';
  import Avatar from '$lib/components/Avatar.svelte';
  import Button from '$lib/components/Button.svelte';
</script>

<header
  class="fixed top-0 flex justify-between z-20 items-center px-12 h-[64px] shadow-md w-full bg-slate-50 text-black dark:bg-neutral-900 dark:text-white"
>
  <a href="/" class="flex items-center justify-center">
    <span class="text-2xl">🪴</span>
    <strong class="text-xl font-semibold">gabble</strong>
  </a>
  <div class="flex gap-4">
    {#if $page.data.user}
      <article class="flex items-center">
        <Avatar dropdown size="sm" />
      </article>
    {:else}
      <Button
        variant="primary"
        on:click={() => (window.location.href = '/login')}>Log in</Button
      >
      <Button on:click={() => (window.location.href = '/signup')}
        >Sign up</Button
      >
    {/if}
  </div>
</header>
