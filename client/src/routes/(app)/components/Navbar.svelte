<script lang="ts">
  import { page } from '$app/stores';
  import Avatar from '$lib/components/Avatar.svelte';
  import Button from '$lib/components/Button.svelte';
  import { clickOutside } from '$lib/actions/click-outside';

  let isDropdownOpen = false;

  const handleDropdownClick = () => {
    isDropdownOpen = !isDropdownOpen;
  };
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
        <button on:click={handleDropdownClick}>
          <Avatar size="sm" />
        </button>
        {#if isDropdownOpen}
          <div class="relative">
            <div
              use:clickOutside
              on:clickOutside={handleDropdownClick}
              id="dropdown"
              class="z-[100] -right-0 top-7 absolute bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700 dark:divide-gray-600"
            >
              <div class="px-4 py-3 text-sm text-gray-900 dark:text-white">
                <div class="truncate">
                  {$page.data.user.name + ' ' + $page.data.user.surname}
                </div>
                <div class="font-medium truncate">
                  {$page.data.user.username}
                </div>
              </div>
              <div class="py-2">
                <a
                  href="/logout"
                  class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white"
                  >Sign out</a
                >
              </div>
            </div>
          </div>
        {/if}
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
