<script lang="ts">
  import { newForm } from "@whizzes/svelte-forms";
  import * as Yup from "yup";
  import { Button, Card, TextField } from "@whizzes/exo";
  import { notifications } from "@whizzes/svelte-notifications";

  import { createHeader } from "$lib/utils/basic-auth";
  import { LoginError, type ErrorMessages } from "./shared";
  import { UserErrorCode } from "$lib/graphql/schema";

  const errorMessages: ErrorMessages = {
    [LoginError.MissingCredentials]: "Please enter your email and password",
    [UserErrorCode.Unauthorized]: "Invalid email or password",
  };

  const { handleSubmit, values, errors, isSubmitting } = newForm({
    initialValues: {
      username: "",
      password: "",
    },
    validationSchema: Yup.object({
      username: Yup.string().email().required(),
      password: Yup.string().required(),
    }),
    onSubmit: async ({ username, password }) => {
      const basicAuth = createHeader(username, password);
      const response = await fetch("/login", {
        method: "POST",
        headers: {
          Authorization: basicAuth,
        },
      });

      if (response.ok) {
        notifications.notifySuccess("Logged in successfully");
        window.location.pathname = "/";
      } else {
        const data = await response.json();

        const errorMessage =
          errorMessages[data.error as keyof typeof errorMessages] ||
          "An error occurred. Please try again later.";

        notifications.notifyFailure(errorMessage);
      }
    },
  });
</script>

<section class="grid grid-cols-12">
  <Card class="self-center space-y-4 col-start-10 col-end-13 h-max w-full">
    <h3 class="text-lg">Create an Account</h3>
    <form on:submit|preventDefault={handleSubmit} class="space-y-4">
      <TextField
        name="username"
        label="Username"
        bind:value={$values.username}
      />
      <TextField
        type="password"
        name="password"
        label="Password"
        bind:value={$values.password}
      />
      <Button type="submit" label="Create Account" />
    </form>
  </Card>
</section>
