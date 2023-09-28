<script lang="ts">
  import { notifications } from '@whizzes/svelte-notifications';

  import { page } from '$app/stores';
  import Avatar from '$lib/components/Avatar.svelte';
  import Button from '$lib/components/Button.svelte';
  import Feed from '$lib/components/Feed/Feed.svelte';
  import PostBox from '$lib/components/PostBox.svelte';
  import UploadModal from '$lib/components/upload-modal/index.svelte';
  import EditUserModal from '$lib/components/modals/EditUserModal.svelte';

  import type { User } from '$lib/graphql/schema';

  let user: User = $page.data.profileUser;
  let currentUser: User = $page.data.user;

  let showAvatarModal = false;
  let showEditUserModal = false;

  function handleUserEdited(event: CustomEvent<{ user: User }>) {
    notifications.notifySuccess('User edited with success');

    user = event.detail.user;
    currentUser = event.detail.user;
    showEditUserModal = false;
  }
</script>

<svelte:head>
  <title>{user.name} (@{user.username})</title>
</svelte:head>
<div class="flex justify-center w-full">
  <div>
    <header class="relative h-fit">
      <img
        width="1000"
        height="1000"
        class="object-cover h-[50vh] md:w-[80vw] md:h-[40vh] xl:h-[50vh]"
        src="https://images.unsplash.com/photo-1569317002804-ab77bcf1bce4?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxzZWFyY2h8Mnx8dW5zcGxhc2h8ZW58MHx8MHx8fDA%3D&w=1000&q=80"
        alt={`${user.username} cover image`}
      />
      <div
        class="absolute w-full -bottom-5 rounded-t-3xl bg-white dark:bg-neutral-900"
      >
        <div class="md:flex justify-around items-end relative bottom-6">
          <div class="md:flex items-end ml-5 md:ml-0">
            <button on:click={() => (showAvatarModal = true)}>
              <Avatar {user} size="3xl" />
            </button>
            <div class="mt-2 md:ml-5">
              <h1 class="text-2xl md:text-4xl font-medium">
                {user.name}
                {user.surname}
              </h1>
              <span class="text text-slate-400">@{user.username}</span>
            </div>
          </div>
          <div class="text-end">
            {#if user.id === currentUser.id}
              <Button
                on:click={() => (showEditUserModal = true)}
                class="mt-6 mr-5 md:mt-0">Edit profile</Button
              >
            {:else}
              <Button variant="primary" class="mt-6 mr-5 md:mt-0">Follow</Button
              >
            {/if}
          </div>
        </div>
        <nav class="border-b-2 border-gray-200 font-medium text-center">
          <ul class="flex flex-wrap -mb-px">
            <li class="mr-2">
              <a
                href="#feed"
                class="inline-block p-4 border-b border-transparent rounded-t-lg hover:text-gray-900 border-gray-400"
                >Feed</a
              >
            </li>
          </ul>
        </nav>
      </div>
    </header>
    <main class="mt-10 lg:w-1/2 space-y-8">
      {#if user.id === currentUser.id}
        <PostBox />
      {/if}
      <Feed username={user.username} />
    </main>
  </div>
</div>

<UploadModal bind:show={showAvatarModal} />
<EditUserModal
  {currentUser}
  bind:show={showEditUserModal}
  on:userEdited={handleUserEdited}
/>
