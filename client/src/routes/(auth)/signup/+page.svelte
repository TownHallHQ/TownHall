<script lang="ts">
  import { newForm } from "@whizzes/svelte-forms";
  import * as Yup from "yup";
  import { notifications } from "@whizzes/svelte-notifications";

  import TextField from "$lib/components/TextField.svelte";
  import Button from "$lib/components/Button.svelte";
  import { UserError, UserService } from "$lib/services/UserService";
  import { getContextClient } from "@urql/svelte";

  import type { CurrentUserFragment } from "$lib/graphql/schema";

  let registeredUser: CurrentUserFragment | null = null;

  const urqlClient = getContextClient();
  const { handleSubmit, values, errors } = newForm({
    initialValues: {
      name: "",
      surname: "",
      username: "",
      email: "",
      password: "",
    },
    validationSchema: Yup.object({
      name: Yup.string().required(),
      email: Yup.string().email().required(),
      username: Yup.string().matches(/^[a-zA-Z][a-zA-Z0-9_]*[a-zA-Z0-9]$/),
      password: Yup.string().required(),
    }),
    onSubmit: async (values) => {
      try {
        const user = await UserService.userRegister(urqlClient, values);

        notifications.notifySuccess(
          `A user with the email ${user.email} has been created!`
        );
        registeredUser = user;
      } catch (err) {
        if (err instanceof UserError) {
          notifications.notifyFailure(err.message);
          return;
        }

        notifications.notifyFailure(
          "An error occurred while creating the user"
        );
      }
    },
  });
</script>

{#if registeredUser}
  <div class="flex flex-col justify-center h-full">
    <div class="text-lg py-4">
      <h1 class="font-semibold">Welcome to {registeredUser.name}!</h1>
      <span> Visit the Login Page to continue </span>
    </div>
  </div>
{:else}
  <div class="flex flex-col justify-center h-full">
    <div class="text-lg py-4">
      <h1 class="font-semibold">Welcome to playa!</h1>
      <span> Create an account to continue </span>
    </div>
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
      <TextField
        id="email"
        name="email"
        type="email"
        placeholder="E.g. user@email.com"
        label="Email"
        error={$errors.email}
        bind:value={$values.email}
      />
      <TextField
        type="password"
        id="password"
        name="password"
        label="Password"
        placeholder="•••••••"
        error={$errors.password}
        bind:value={$values.password}
      />
      <Button type="submit" variant="primary">Sign up</Button>
    </form>
    <small class="text-gray-600">
      Already have an account? <a class="text-blue-600 underline" href="/login"
        >Log in</a
      >
    </small>
  </div>
{/if}
