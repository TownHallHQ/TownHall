<script lang="ts">
  import { newForm } from '@whizzes/svelte-forms';
  import * as Yup from 'yup';
  import { notifications } from '@whizzes/svelte-notifications';

  import { createHeader } from '$lib/utils/basic-auth';
  import { LoginError, type ErrorMessages } from './shared';
  import { UserErrorCode } from '$lib/graphql/schema';
  import Card from '$lib/components/Card.svelte';
  import TextField from '$lib/components/TextField.svelte';
  import Button from '$lib/components/Button.svelte';

  const errorMessages: ErrorMessages = {
    [LoginError.MissingCredentials]: 'Please enter your email and password',
    [UserErrorCode.Unauthorized]: 'Invalid email or password',
  };

  const { handleSubmit, values, errors, isSubmitting } = newForm({
    initialValues: {
      username: '',
      password: '',
    },
    validationSchema: Yup.object({
      username: Yup.string().email().required(),
      password: Yup.string().required(),
    }),
    onSubmit: async ({ username, password }) => {
      const basicAuth = createHeader(username, password);
      const response = await fetch('/login', {
        method: 'POST',
        headers: {
          Authorization: basicAuth,
        },
      });

      if (response.ok) {
        notifications.notifySuccess('Logged in successfully');
        window.location.pathname = '/';
      } else {
        const data = await response.json();

        const errorMessage =
          errorMessages[data.error as keyof typeof errorMessages] ||
          'An error occurred. Please try again later.';

        notifications.notifyFailure(errorMessage);
      }
    },
  });
</script>

<h3 class="text-2xl">Login into your account</h3>
<form
  on:submit|preventDefault={handleSubmit}
  class="flex flex-col w-full mt-8 space-y-1"
>
  <TextField
    name="username"
    label="Username"
    placeholder="johndoe"
    bind:value={$values.username}
  />
  <TextField
    type="password"
    name="password"
    label="Password"
    placeholder="• • • • • • • •"
    bind:value={$values.password}
    error={$errors.password}
  />

  <div class="flex justify-between">
    <Button type="submit" disabled={$isSubmitting} variant="primary"
      >Login</Button
    >
    <Button
      disabled={$isSubmitting}
      on:click={() => {
        window.location.href = '/signup';
      }}>Create an account</Button
    >
  </div>
</form>
