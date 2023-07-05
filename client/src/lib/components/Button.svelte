<script lang="ts">
  import classNames from 'classnames';

  export let type: 'button' | 'submit' = 'button';
  export let fullWidth = false;
  export let disabled = false;

  let customClassNames = '';
  export { customClassNames as class };

  let className = classNames(
    customClassNames,
    'bg-black px-5 py-3 text-base font-medium text-center text-white bg-primary-700 rounded-lg hover:bg-primary-800 focus:ring-4 focus:ring-primary-300 sm:w-auto dark:bg-primary-600 dark:hover:bg-primary-700 dark:focus:ring-primary-800',
    {
      'w-full': fullWidth,
      '!cursor-not-allowed !opacity-50 !bg-gray-700 !text-white': disabled
    }
  );
</script>

<button {type} class={className} {disabled} on:click>
  <slot />
</button>
