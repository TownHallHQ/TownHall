<script lang="ts">
  import { page } from '$app/stores';
  import Avatar from '$lib/components/Avatar.svelte';
  import Button from '$lib/components/Button.svelte';
</script>

<header
  class="fixed top-0 flex justify-between items-center px-12 bg-white h-[64px] shadow-md w-full"
>
  <figure>
    <img
      src="https://via.placeholder.com/120x40"
      alt="Nexus Logo"
      height="40"
      width="120"
    />
  </figure>
  <div class="flex gap-4">
    {#if $page.data.user}
      <article class="flex items-center">
        <Avatar dropdown size="sm" />
        <span class="pl-2">
          Hi, {$page.data.user.username}!
        </span>
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
