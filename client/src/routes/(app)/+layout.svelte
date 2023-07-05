<script lang="ts">
  import Navbar from './components/Navbar/Navbar.svelte';
  import Sidebar from './components/Sidebar/Sidebar.svelte';
</script>

<div>
  <Navbar />
  <div class="flex pt-16 overflow-hidden bg-gray-50 dark:bg-gray-900">
    <Sidebar />
    <div
      id="main-content"
      class="relative w-full h-full overflow-y-auto bg-gray-50 lg:ml-64 dark:bg-gray-900"
    >
      <main class="dark:text-white min-h-[calc(100vh-64px)]">
        <slot />
      </main>
    </div>
  </div>
</div>
