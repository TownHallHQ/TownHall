<script lang="ts">
  import Navbar from './components/Navbar/Navbar.svelte';
</script>

<div>
  <Navbar />
  <div class="flex pt-16 bg-gray-50 dark:bg-gray-900">
    <main class="dark:text-white min-h-[calc(100vh-64px)]">
      <slot />
    </main>
  </div>
</div>
