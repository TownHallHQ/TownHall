<script lang="ts">
  import DesktopIcon from "$lib/icons/desktop-icon.svelte";
  let input: HTMLInputElement;
</script>

<input
  class="dark:bg-gray-600 py-3 px-3 w-full"
  type="file"
  name="file"
  id="file"
  hidden
  on:change
  bind:this={input}
/>
<button
  class="grid place-items-center w-full h-40 p-4 rounded-xl dark:border-white border mx-auto border-dashed"
  on:click={() => input.click()}
>
  <div class="w-16 h-16 rounded-full p-4 border dark:border-white">
    <DesktopIcon />
  </div>
  <div class="opacity-70">Click here to upload avatar</div>
</button>
