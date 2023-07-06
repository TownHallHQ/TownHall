<script>
  import Card from '$lib/components/Card.svelte';
</script>

<div class="flex bg-gray-900 min-h-screen justify-center items-center">
  <Card class="h-max w-full mx-2 md:w-1/2 xl:w-1/4">
    <slot />
  </Card>
</div>
