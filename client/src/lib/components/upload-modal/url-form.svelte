<script lang="ts">
  import UrlIcon from "$lib/icons/url-icon.svelte";
  import { createEventDispatcher } from "svelte";
  import Button from "../Button.svelte";
  let url: string = "";
  const dispatch = createEventDispatcher();

  $: console.log(url);
</script>

<article class="flex flex-col gap-6 items-center">
  <div class="w-16 h-16 p-4 rounded-full dark:border-white border">
    <UrlIcon />
  </div>
  <div class="flex flex-col gap-4 w-full max-w-sm">
    <input
      class="dark:bg-neutral-600 py-3 px-3 w-full"
      type="url"
      name="url"
      id="url"
      placeholder="Paste the Image URL here"
      bind:value={url}
    />

    <Button on:click={() => dispatch("urlLoad", { url })} variant="primary"
      >Load</Button
    >
  </div>
</article>
