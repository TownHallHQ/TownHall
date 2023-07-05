<script lang="ts">
  import type { SvelteComponentDev } from 'svelte/internal';

  export let icon: SvelteComponentDev | any;
  export let text: string;
  export let href: string;
</script>

<li>
  <a
    {href}
    class="flex items-center p-2 text-base text-gray-900 rounded-lg hover:bg-gray-100 group dark:text-gray-200 dark:hover:bg-gray-700"
  >
    <svelte:component this={icon} class="h-6 w-6" />
    <span class="ml-3">{text}</span>
  </a>
</li>
