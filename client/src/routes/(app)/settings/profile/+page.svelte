<script lang="ts">
  import { UserService } from "$lib/services/UserService";
  import { userStore } from "$lib/stores/user";
  import { getContextClient } from "@urql/svelte";
    import { notifications } from "@whizzes/svelte-notifications";

  let fileInputEl: HTMLInputElement;

  const urqlClient = getContextClient();

  async function handleUploadAvatar() {
    if (fileInputEl.files?.length && fileInputEl.files[0]) {
      await userStore.updateAvatar(urqlClient, fileInputEl.files[0]);
      notifications.notifySuccess("Avatar updated");
    }
  }
</script>

<h1>Profile</h1>

<button on:click={() => fileInputEl.click()}>
  <figure>
    <img
      src={$userStore?.avatar?.url}
      alt="{$userStore?.username} avatar"
      height="40"
      width="40"
    />
  </figure>
</button>
<input
  hidden
  type="file"
  name=""
  id=""
  bind:this={fileInputEl}
  on:change={handleUploadAvatar}
/>
