<script lang="ts">
  import Card from '$lib/components/Card.svelte';
  import type { Post } from '$lib/graphql/schema';

  import Avatar from './Navbar/Avatar.svelte';
  export let post: Post;
</script>

<Card>
  <header class="flex items-center justify-start">
    <Avatar user={post.author} size="md" />
    <div class="flex flex-col px-2">
      <h1 class="text-gray-800">
        {post.author.name}
        {post.author.surname}
        <span class="text-slate-400">@{post.author.username}</span>
      </h1>
      <time class="text-gray-400 text-sm" datetime={post.createdAt}
        >{new Date(post.createdAt).toLocaleDateString('en-us', {
          weekday: 'long',
          year: 'numeric',
          month: 'short',
          day: 'numeric',
        })}</time
      >
    </div>
  </header>
  <main class="pt-4">
    <h1 class="text-2xl font-medium">{post.title}</h1>
    <p class="text-lg">{post.content}</p>
  </main>
</Card>
