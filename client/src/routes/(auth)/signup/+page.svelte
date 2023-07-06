<script lang="ts">
  import { newForm } from '@whizzes/svelte-forms';
  import * as Yup from 'yup';
  import { notifications } from '@whizzes/svelte-notifications';

  import TextField from '$lib/components/TextField.svelte';
  import Button from '$lib/components/Button.svelte';

  const { handleSubmit, values, errors, isSubmitting } = newForm({
    initialValues: {
      name: '',
      surname: '',
      username: '',
      email: '',
      password: '',
    },
    validationSchema: Yup.object({
      name: Yup.string().required(),
      email: Yup.string().email().required(),
      username: Yup.string().matches(/^[a-zA-Z][a-zA-Z0-9_]*[a-zA-Z0-9]$/),
      password: Yup.string().required(),
    }),
    onSubmit: async (values) => {
      const response = await fetch('/signup', {
        method: 'POST',
        body: JSON.stringify(values),
      });

      if (response.ok) {
        notifications.notifySuccess('Account created');
        window.location.pathname = '/';
      } else {
        notifications.notifyFailure('Error creating account, please try again');
      }
    },
  });
</script>

<h2 class="text-2xl font-bold text-gray-900 dark:text-white">
  Create an account
</h2>
<form class="flex flex-col w-full mt-8 space-y-1" on:submit={handleSubmit}>
  <TextField
    id="name"
    name="name"
    type="text"
    placeholder="E.g. John"
    label="Name"
    bind:value={$values.name}
    error={$errors.name}
  />
  <TextField
    id="surname"
    name="surname"
    type="text"
    placeholder="E.g. Appleseed"
    label="Last name"
    bind:value={$values.surname}
    error={$errors.surname}
  />
  <TextField
    id="username"
    name="username"
    type="text"
    placeholder="E.g. johndoe"
    label="Username"
    bind:value={$values.username}
    error={$errors.username}
  />
  <TextField
    id="email"
    name="email"
    type="email"
    placeholder="E.g. user@email.com"
    label="Email"
    bind:value={$values.email}
    error={$errors.email}
  />
  <TextField
    type="password"
    id="password"
    name="password"
    label="Password"
    placeholder="• • • • • • • •"
    bind:value={$values.password}
    error={$errors.password}
  />
  <div class="flex justify-between">
    <Button variant="primary" type="submit" disabled={$isSubmitting}
      >Create account</Button
    >
    <Button
      on:click={() => (window.location.href = '/login')}
      disabled={$isSubmitting}>Login into your account</Button
    >
  </div>
</form>
