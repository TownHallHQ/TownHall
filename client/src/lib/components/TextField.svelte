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
    'text-md  font-semibold bg-gray-50 px-3 py-3 placeholder-slate-300 text-slate-600 rounded-lg text-sm outline-none focus:outline-none focus:ring w-full'
  );
  const handleInput = (event: Event): void => {
    const target = event.target as HTMLInputElement;
    value = type.match(/^(number|range)$/) ? +target.value : target.value;
  };
</script>

{#if label}
  <label
    for={name}
    class:text-red-600={!!error}
    class="block text-sm font-medium text-gray-700 pb-2">{label}</label
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
<p class:opacity-0={!error} class="text-sm pt-1 text-red-600">{error}</p>
