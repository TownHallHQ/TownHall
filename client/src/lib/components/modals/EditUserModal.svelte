<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { newForm } from '@whizzes/svelte-forms';
  import { notifications } from '@whizzes/svelte-notifications';
  import { getContextClient } from '@urql/svelte';
  import * as Yup from 'yup';

  import { UserError, UserService } from '$lib/services/UserService';
  import TextField from '../TextField.svelte';
  import Button from '../Button.svelte';
  import Modal from '../ui/modal.svelte';

  import type { User } from '$lib/graphql/schema';

  const dispatch = createEventDispatcher();
  const urqlClient = getContextClient();

  export let show = false;
  export let currentUser: User;

  let modalReference: HTMLDialogElement;

  const { handleSubmit, values, errors } = newForm({
    initialValues: {
      name: currentUser.name,
      surname: currentUser.surname,
    },
    validationSchema: Yup.object({
      name: Yup.string().required(),
      surname: Yup.string(),
    }),
    async onSubmit(values) {
      try {
        const user = await UserService.update(
          urqlClient,
          currentUser.id,
          values
        );
        modalReference.close();
        dispatch('userEdited', { user });
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

<Modal bind:dialog={modalReference} bind:show>
  <header slot="header" class="flex justify-between p-4 -mb-5">
    <h1 class="text-xl font-semibold">Edit profile</h1>
  </header>
  <form class="flex flex-col py-4 space-y-4" on:submit={handleSubmit}>
    <TextField
      id="name"
      name="name"
      type="text"
      placeholder={currentUser.name}
      label="Name"
      error={$errors.name}
      bind:value={$values.name}
    />
    <TextField
      id="surname"
      name="surname"
      type="text"
      placeholder={currentUser.surname}
      label="Surname"
      error={$errors.surname}
      bind:value={$values.surname}
    />
    <Button type="submit" variant="primary">Save</Button>
  </form>
</Modal>
