<script lang="ts">
  import Card from '$lib/components/Card.svelte';

  import Avatar from './Navbar/Avatar.svelte';
  export let post: any = {
    id: 1,
    title: 'title',
    content: 'body',
    createdAt: '24/12/2023',
    user: {
      id: 1,
      name: 'Michael',
      surname: 'Liendo',
      username: 'MichaelLiendo',
    },
  };
</script>

<Card>
  <header class="flex items-center justify-start">
    <Avatar user={post.user} size="md" />
    <div class="flex flex-col px-2">
      <strong class="text-gray-800">{post.user.username}</strong>
      <span class="text-gray-400 text-sm"
        >{new Date(post.createdAt).toLocaleDateString()}</span
      >
    </div>
  </header>
  <main class="pt-4">
    <h1 class="text-2xl font-bold">{post.title}</h1>
    <p>{post.content}</p>
  </main>
</Card>
