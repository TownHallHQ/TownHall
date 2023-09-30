<script lang="ts">
  import { userStore } from "$lib/stores/user";
</script>

<article class="flex items-center">
  <figure>
    <img src="{$userStore?.avatar?.url}" alt="{$userStore?.username} avatar" height="40" width="40" />
  </figure>
</article>
