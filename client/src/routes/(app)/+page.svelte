<script lang="ts">
  import { Feed } from '$lib/components/Feed';
  import PostBox from '$lib/components/PostBox.svelte';

  import type { PostCreateFieldsFragment } from '$lib/graphql/schema';

  function handlePostCreated(event: CustomEvent<{ post: PostCreateFieldsFragment; }>) {
    console.log('post created', event.detail);
  }
</script>

<div class="grid gap-6 pt-6 grid-cols-4 md:grid-cols-12 w-full self-start">
  <PostBox class="col-span-4 md:col-span-6 md:col-start-4" on:postCreated={handlePostCreated} />
  <Feed class="self-start col-span-4 md:col-span-6 md:col-start-4" />
</div>
