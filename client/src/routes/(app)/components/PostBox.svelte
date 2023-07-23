<script lang="ts">
  import { page } from '$app/stores';
  import Button from '$lib/components/Button.svelte';
  import Card from '$lib/components/Card.svelte';
  import TextField from '$lib/components/TextField.svelte';
  import Avatar from './Navbar/Avatar.svelte';

  let customClassName: string | null = null;
  export { customClassName as class };
</script>

<Card class={customClassName}>
  <form>
    <div class="flex space-x-4">
      <Avatar size="sm" user={$page.data.user} />
      <div class="w-full">
        <TextField
          class="w-full"
          name="title"
          type="text"
          placeholder="My post"
        />
      </div>
    </div>
    <TextField
      class="w-[calc(100%-44px)] mt-3"
      name="content"
      type="text"
      placeholder="Whats on your mind?"
    />
    <div class="flex mt-2 justify-end">
      <Button type="submit" variant="primary">Publish</Button>
    </div>
  </form>
</Card>
