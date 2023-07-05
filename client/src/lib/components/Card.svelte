<script lang="ts">
  import classNames from 'classnames';

  let className = '';
  export { className as class };
</script>

<div
  class={classNames(
    'p-4 bg-white border border-gray-200 rounded-lg shadow-sm dark:border-gray-700 sm:p-6 dark:bg-gray-800',
    className
  )}
>
  <slot />
</div>
