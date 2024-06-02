<script lang="ts">
  import { userStore } from '$lib/stores/user';
  import UserSummary from './UserSummary.svelte';
</script>

<header
  class="fixed top-0 flex justify-between z-20 items-center px-12 h-[64px] border-b w-full bg-slate-50 text-inner-light dark:bg-neutral-900 dark:text-white"
>
  <a href="/" class="flex items-center justify-center">
    <span class="text-2xl">🏖️</span>
    <strong class="text-xl font-semibold">townhall</strong>
  </a>
  <div class="flex gap-4">
    {#if $userStore}
      <UserSummary />
    {/if}
  </div>
</header>
