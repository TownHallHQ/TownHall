<script lang="ts">
  import classNames from 'classnames';

  let className: string = '';
  export { className as class };

  let finalClass = classNames(className, 'bg-white dark:bg-neutral-900 rounded-lg p-4 w-full');
</script>

<div class={finalClass}>
  <slot />
</div>
