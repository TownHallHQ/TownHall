<script lang="ts">
  // @ts-ignore
  import { useRegisterSW } from "virtual:pwa-register/svelte";
  const { needRefresh, updateServiceWorker, offlineReady } = useRegisterSW({
    onRegistered(r: any) {
      console.log(`SW Registered: ${r}`);
    },
    onRegisterError(error: any) {
      console.log("SW registration error", error);
    },
  });
  const close = () => {
    offlineReady.set(false);
    needRefresh.set(false);
  };
  $: toast = $offlineReady || $needRefresh;
</script>

{#if toast}
  <div
    class="fixed right-0 bottom-0 m-4 p-3 border border-gray-600 dark:border-gray-400 z-2 text-left shadow bg-white dark:bg-neutral-700 dark:text-white"
    role="alert"
  >
    <div class="mb-2">
      {#if $offlineReady}
        <span> App ready to work offline </span>
      {:else}
        <span> New content available, click on reload button to update. </span>
      {/if}
    </div>
    <div class="flex items-center gap-2">
      {#if $needRefresh}
        <button
          class="py-1 px-3 border border-gray-600 dark:border-gray-400"
          on:click={() => updateServiceWorker(true)}
        >
          Reload
        </button>
      {/if}
      <button
        class="py-1 px-3 border border-gray-600 dark:border-gray-400"
        on:click={close}
      >
        Close
      </button>
    </div>
  </div>
{/if}
