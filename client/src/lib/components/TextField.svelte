<script lang="ts">
  import Warning from "~icons/custom/warning";

  export let type: "text" | "number" | "email" | "password" | "date" = "text";
  export let name: string;
  export let id = "";
  export let error: string | null = null;
  export let value: string | number | Date | null = null;
  export let label: string | null = null;
  export let placeholder: string | undefined = undefined;
  export let required = false;
  export let autocomplete: "true" | "false" | undefined = undefined;
  let className = "";
  export { className as class };

  const handleInput = (event: Event): void => {
    const target = event.target as HTMLInputElement;
    value = type.match(/^(number|range)$/) ? +target.value : target.value;
  };
</script>

{#if label}
  <label
    for={name}
    class:underline={!!error}
    class:text-rose-600={!!error}
    class="block text-sm font-medium text-gray-700">{label}</label
  >
{/if}

<div class="input_wrapper">
  <div class="input_container">
    <input
      {required}
      {autocomplete}
      id={id || name}
      {type}
      {name}
      {value}
      {placeholder}
      class="input{className ? ' ' + className : ''}"
      class:input_error={!!error}
      on:input={handleInput}
    />
    {#if error}
    <figure class="icon_error_icon_container">
      <Warning class="input_error_icon" />
    </figure>
    {/if}
  </div>
  {#if error}
    <p class="input_error_message">{error}</p>
  {/if}
</div>

<style lang="postcss">
  .input_wrapper {
    @apply flex flex-col;
  }

  .input_container {
    @apply flex relative;
  }

  .input_error_icon {
    @apply text-rose-600 h-4 w-4;
  }

  .icon_error_icon_container {
    @apply flex items-center justify-center p-4 pointer-events-none;
    @apply absolute right-0 top-0;
  }

  .input_error_message {
    @apply text-rose-600 text-sm;
  }

  .input {
    @apply bg-gray-200 border border-transparent rounded text-gray-800;
    @apply w-full p-3 text-[.9rem];
  }

  .input:active,
  .input:focus {
    @apply outline-none border-gray-400;
  }

  .input_error,
  .input_error:active,
  .input_error:focus {
    @apply border-rose-600 text-rose-600;
  }
</style>
