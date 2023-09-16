<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import Button from "../Button.svelte";
  const dispatch = createEventDispatcher();
  export let url: string;
</script>

<div class="w-full h-full">
  <img src={url} alt="Preview" class="w-full h-full aspect-video" />
</div>

<div class="flex justify-center mt-4">
  <Button on:click={() => dispatch("fileUpload")} variant="primary"
    >Upload</Button
  >
</div>
