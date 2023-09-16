<script lang="ts">
  import { createEventDispatcher } from "svelte";
  export let text: string;
  export let cardType: string;
  const dispatch = createEventDispatcher();
</script>

<button on:click={() => dispatch("uploadFrom", { cardType })}>
  <article
    class="dark:bg-neutral-600 rounded-xl flex flex-col gap-4 p-6 items-center dark:hover:bg-neutral-500"
  >
    <div class="w-16 h-16 p-4 rounded-full dark:border-white border">
      <slot name="icon" />
    </div>
    <p>{text}</p>
  </article>
</button>
