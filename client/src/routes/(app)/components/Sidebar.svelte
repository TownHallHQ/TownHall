<script lang="ts">
  import { page } from '$app/stores';

  import Profile from './Navbar/Avatar.svelte';
  import Button from '$lib/components/Button.svelte';
  import Divider from '$lib/components/Divider.svelte';
</script>

<aside
  class="bg-slate-50 border-r border-slate-200 w-96 flex flex-col p-2 text-black"
>
  <div class="flex items-center">
    <Profile user={$page.data.user} />
    <span class="flex ml-2"
      >{$page.data.user.name} {$page.data.user.surname}</span
    >
  </div>
  <Divider />
  <div class="flex flex-col justify-start">
    <Button fullWidth variant="primary">Publish</Button>
  </div>
</aside>
