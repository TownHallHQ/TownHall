<script lang="ts">
  import { page } from '$app/stores';
  import Feed from './components/Feed.svelte';
  import PostBox from './components/PostBox.svelte';
  import Sidebar from './components/Sidebar.svelte';
</script>

<Sidebar />

<div class="px-4 pt-6 space-y-6 sm:w-full lg:w-1/2">
  Welcome back {$page.data.user.name}!
  <PostBox />

  <Feed />
</div>
