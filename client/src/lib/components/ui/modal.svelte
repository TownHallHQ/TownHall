<script lang="ts">
  export let show: boolean = false;
  export let dialog: HTMLDialogElement;
  $: if (dialog && show) dialog.showModal();
</script>

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions -->
<dialog
  bind:this={dialog}
  on:close={() => (show = false)}
  on:click|self={() => dialog.close()}
  class="max-w-lg w-full rounded-md dark:backdrop:bg-neutral-800/60 dark:bg-neutral-700 dark:text-white"
>
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div on:click|stopPropagation>
    {#if $$slots.header}
      <header>
        <slot name="header" />
      </header>
    {/if}
    <div class="p-4">
      <slot />
    </div>
    <!-- svelte-ignore a11y-autofocus -->
    <footer class="flex items-center justify-end p-4">
      <button
        class="dark:bg-neutral-600 px-4 py-2 rounded-full dark:hover:bg-neutral-500"
        autofocus
        on:click={() => dialog.close()}>Close</button
      >
    </footer>
  </div>
</dialog>
