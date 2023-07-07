<script lang="ts">
  import classNames from 'classnames';

  let customClassName: string | null = null;
  export { customClassName as class };

  let className = classNames(customClassName, 'bg-white rounded-lg p-4 w-full');
</script>

<div class={className}>
  <slot />
</div>
