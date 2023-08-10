<script lang="ts">
  import Navbar from './components/Navbar.svelte';
  import Sidebar from './components/Sidebar.svelte';
</script>

<div>
  <Navbar />
  <div class="flex pt-16 overflow-hidden bg-gray-50 dark:bg-gray-900">
    <Sidebar />
    <div
      id="main-content"
      class="w-full min-h-[calc(100vh-64px)] overflow-y-auto bg-gray-50"
    >
      <slot />
    </div>
  </div>
</div>
