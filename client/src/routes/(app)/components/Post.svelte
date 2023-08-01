<script lang="ts">
  import Card from '$lib/components/Card.svelte';
  import type { Post } from '$lib/graphql/schema';

  // import Avatar from './Navbar/Avatar.svelte';
  export let post: Post;
</script>

<Card>
  <header class="flex items-center justify-start">
    <!-- <Avatar user={{ name: 'John', surname: 'Doe' }} size="md" /> -->
    <div class="flex flex-col px-2">
      <h1 class="text-gray-800">{post.title}</h1>
      <span class="text-gray-400 text-sm"
        >{new Date(post.createdAt).toLocaleDateString()}</span
      >
    </div>
  </header>
  <main class="pt-4">
    <p class="text-lg">{post.content}</p>
  </main>
</Card>
