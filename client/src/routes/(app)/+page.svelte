<script lang="ts">
  import { page } from '$app/stores';
  import PostBox from './components/PostBox.svelte';
</script>

<div class="px-4 pt-6 space-y-6 sm:w-full lg:w-1/2">
  Welcome back {$page.data.user.name}!
  <PostBox />
</div>
