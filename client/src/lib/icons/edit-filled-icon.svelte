<script lang="ts">
  export let width: string | number = '24';
  export let height: string | number = '24';
  export let color: string = 'black';
  let className: string = '';
  export { className as class };
</script>

<svg
  class={className}
  xmlns="http://www.w3.org/2000/svg"
  {width}
  {height}
  viewBox={`0 0 24 24`}
  fill="none"
>
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M6 15.5017V18.0017H8.5L15.8733 10.6283L13.3733 8.12833L6 15.5017ZM17.8067 8.695C18.0667 8.435 18.0667 8.015 17.8067 7.755L16.2467 6.195C15.9867 5.935 15.5667 5.935 15.3067 6.195L14.0867 7.415L16.5867 9.915L17.8067 8.695Z"
    fill={color}
  />
</svg>
