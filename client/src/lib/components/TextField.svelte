<script lang="ts">
  import classNames from 'classnames';

  export let type: 'text' | 'number' | 'email' | 'password' | 'date' = 'text';
  export let name: string;
  export let id = '';
  export let error: string | null = null;
  export let value: string | number | Date | null = null;
  export let label: string | null = null;
  export let placeholder: string | undefined = undefined;
  export let required = false;
  export let autocomplete: 'true' | 'false' | undefined = undefined;
  let customClassNames = '';
  export { customClassNames as class };
  let className = classNames(
    customClassNames,
    'bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-primary-500 focus:border-primary-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-primary-500 dark:focus:border-primary-500'
  );
  const handleInput = (event: Event): void => {
    const target = event.target as HTMLInputElement;
    value = type.match(/^(number|range)$/) ? +target.value : target.value;
  };
</script>

{#if label}
  <label
    for={name}
    class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
    >{label}</label
  >
{/if}

<input
  {name}
  {id}
  {placeholder}
  {required}
  {autocomplete}
  {type}
  {value}
  class:border-red-600={!!error}
  class={className}
  on:change
  on:blur
  on:input={handleInput}
/>
<p class:opacity-0={!error} class="text-sm pb-1 text-red-600">{error}</p>
