<script lang="ts">
  import Avatar from '$lib/components/Avatar.svelte';
  import Card from '$lib/components/Card.svelte';

  import type { Post } from '$lib/graphql/schema';

  export let post: Post;
</script>

<Card>
  <header class="flex items-center justify-start">
    <Avatar redirect user={post.author} size="md" />
    <div class="flex flex-col px-2">
      <h1 class="text-neutral-600 dark:text-neutral-200">
        {post.author.name}
        {post.author.surname}
        <span class="text-neutral-400 dark:text-neutral-50"
          >@{post.author.username}</span
        >
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
    <h1 class="text-xl text-neutral-800 dark:text-neutral-50 font-medium">
      {post.title}
    </h1>
    {#if post.content}
      <p class="text-neutral-600 dark:text-neutral-200">{post.content}</p>
    {/if}
  </main>
</Card>
