<script lang="ts">
  import classNames from 'classnames';

  export let type: 'button' | 'submit' = 'button';
  export let variant: 'primary' | null = null;
  export let fullWidth = false;
  export let disabled = false;
  export let isLoading = false;

  let customClassNames = '';
  export { customClassNames as class };

  let className = classNames(
    customClassNames,
    'border py-2 px-6 rounded-full text-sm flex items-center font-semibold',
    {
      'w-full': fullWidth,
      '!opacity-50 !text-gray-400': isLoading || disabled,
      '!cursor-not-allowed': disabled,
      '!cursor-progress': isLoading,
      'border-gray-200 text-gray-600 hover:bg-gray-100': !variant,
      '!text-white border-blue-600 bg-blue-600 hover:bg-blue-800 hover:!border-blue-800':
        variant === 'primary',
    }
  );
</script>

<button {type} class={className} {disabled} on:click>
  <slot />
</button>
