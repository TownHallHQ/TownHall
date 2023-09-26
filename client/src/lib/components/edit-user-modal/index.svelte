<script lang="ts">
  import { newForm } from '@whizzes/svelte-forms';
  import * as Yup from 'yup';
  import { page } from '$app/stores';

  import Modal from '../ui/modal.svelte';
  import TextField from '../TextField.svelte';
  import type { User } from '$lib/graphql/schema';
  import Button from '../Button.svelte';

  export let show = false;

  const currentUser: User = $page.data.user;

  const { handleSubmit, values, errors } = newForm({
    initialValues: {
      name: currentUser.name,
      surname: currentUser.surname,
      username: currentUser.username,
    },
    validationSchema: Yup.object({
      name: Yup.string().required(),
      surname: Yup.string(),
      username: Yup.string().matches(/^[a-zA-Z][a-zA-Z0-9_]*[a-zA-Z0-9]$/),
    }),
    onSubmit(values, helpers) {
      console.log(values);
    },
  });
</script>

<Modal bind:show>
  <header slot="header" class="flex justify-between p-4 -mb-5">
    <h1 class="text-xl font-semibold">Edit profile</h1>
    <Button variant="primary">Save</Button>
  </header>

  <form class="flex flex-col py-4 space-y-4" on:submit={handleSubmit}>
    <TextField
      id="name"
      name="name"
      type="text"
      placeholder="E.g. John"
      label="Name"
      error={$errors.name}
      bind:value={$values.name}
    />
    <TextField
      id="surname"
      name="surname"
      type="text"
      placeholder="E.g. Appleseed"
      label="Surname"
      error={$errors.surname}
      bind:value={$values.surname}
    />
    <TextField
      id="username"
      name="username"
      type="text"
      placeholder="E.g. johndoe"
      label="Username"
      error={$errors.username}
      bind:value={$values.username}
    />
  </form>
</Modal>
