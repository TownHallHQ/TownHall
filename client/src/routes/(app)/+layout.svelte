<script lang="ts">
  import { page } from '$app/stores';
  import { userStore } from '$lib/stores/user';
  import Navbar from './components/Navbar';

  userStore.init($page.data.user);
</script>

<div>
  <Navbar />
  <div
    class="pt-16 overflow-hidden bg-base-light text-black dark:bg-neutral-800 dark:text-white"
  >
    <div id="main-content" class="min-h-[calc(100vh-64px)] overflow-y-auto">
      <slot />
    </div>
  </div>
</div>
