<script lang="ts">
  import { page } from '$app/stores';
</script>

<div class="px-4 pt-6 space-y-6">
  Welcome back {$page.data.user.name}!
</div>
