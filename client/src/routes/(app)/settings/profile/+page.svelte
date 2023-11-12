<script lang="ts">
  import { newForm } from '@whizzes/svelte-forms';
  import { notifications } from '@whizzes/svelte-notifications';
  import { getContextClient } from '@urql/svelte';
  import * as Yup from 'yup';

  import Button from '$lib/components/Button.svelte';
  import TextField from '$lib/components/TextField.svelte';
  import { UserError } from '$lib/services/UserService';
  import { userStore } from '$lib/stores/user';

  let fileInputEl: HTMLInputElement;

  const urqlClient = getContextClient();

  async function handleUploadAvatar() {
    if (fileInputEl.files?.length && fileInputEl.files[0]) {
      await userStore.updateAvatar(urqlClient, fileInputEl.files[0]);
      notifications.notifySuccess('Avatar updated');
    }
  }

  const { handleSubmit, values, errors } = newForm({
    initialValues: {
      name: $userStore?.name,
      surname: $userStore?.surname,
    },
    validationSchema: Yup.object({
      name: Yup.string().required(),
      surname: Yup.string(),
    }),
    async onSubmit(values) {
      try {
        await userStore.update(urqlClient, values);
        notifications.notifySuccess('Profile updated');
      } catch (error) {
        console.log(error);

        if (error instanceof UserError) {
          return notifications.notifyFailure(error.message);
        }
        notifications.notifyFailure('Something went wrong');
      }
    },
  });
</script>

<svelte:head>
  <title>Your Profile | {$userStore?.name}</title>
</svelte:head>

<input
  hidden
  type="file"
  name=""
  id=""
  bind:this={fileInputEl}
  on:change={handleUploadAvatar}
/>

<header class="mt-10 mx-3 sm:mx-20 md:mx-56 xl:mx-96 border-b-2 pb-2">
  <h1 class="text-3xl font-semibold">Edit profile</h1>
</header>

<form
  class="mt-6 mx-3 sm:mx-20 md:mx-56 xl:mx-96 space-y-4"
  on:submit={handleSubmit}
>
  <div class="md:flex">
    <div class="flex justify-center lg:block md:w-96">
      <button type="button" on:click={() => fileInputEl.click()}>
        <figure>
          <img
            src={$userStore?.avatar?.url}
            alt="{$userStore?.username} avatar"
            class="rounded-full w-52 h-52 object-cover"
            height="200"
            width="200"
          />
          <figcaption class="text-center mt-2">Change avatar</figcaption>
        </figure>
      </button>
    </div>
    <div class="w-full">
      <TextField
        class="w-full"
        id="name"
        name="name"
        type="text"
        placeholder={$userStore?.name}
        label="Name"
        error={$errors.name}
        bind:value={$values.name}
      />
      <TextField
        class="w-full"
        id="surname"
        name="surname"
        type="text"
        placeholder={$userStore?.surname}
        label="Surname"
        error={$errors.surname}
        bind:value={$values.surname}
      />
    </div>
  </div>
  <div class="flex justify-end">
    <Button class="text-lg" type="submit" variant="primary">Save</Button>
  </div>
</form>
