<script lang="ts">
  import { newForm } from '@whizzes/svelte-forms';
  import * as Yup from 'yup';
  import { notifications } from '@whizzes/svelte-notifications';

  import { createHeader } from '$lib/utils/basic-auth';
  import { LoginError, type ErrorMessages } from './shared';
  import { UserErrorCode } from '$lib/graphql/schema';
  import TextField from '$lib/components/TextField.svelte';
  import Button from '$lib/components/Button.svelte';

  const errorMessages: ErrorMessages = {
    [LoginError.MissingCredentials]: 'Please enter your email and password',
    [UserErrorCode.Unauthorized]: 'Invalid email or password',
  };

  const { handleSubmit, values, errors, isSubmitting } = newForm({
    initialValues: {
      email: '',
      password: '',
    },
    validationSchema: Yup.object({
      email: Yup.string().email().required('Email is required'),
      password: Yup.string().required('Password is required'),
    }),
    onSubmit: async ({ email, password }) => {
      const basicAuth = createHeader(email, password);
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
    name="email"
    type="email"
    label="Email"
    placeholder="john@example.com"
    bind:value={$values.email}
    error={$errors.email}
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
