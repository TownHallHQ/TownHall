<script lang="ts">
  import Navbar from './components/Navbar.svelte';
</script>

<div>
  <Navbar />
  <div class="flex pt-16 overflow-hidden bg-slate-50 text-black dark:bg-neutral-800 dark:text-white">
    <div
      id="main-content"
      class="flex w-full min-h-[calc(100vh-64px)] overflow-y-auto"
    >
      <slot />
    </div>
  </div>
</div>
