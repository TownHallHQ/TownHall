<script lang="ts">
  import { newForm } from "@whizzes/svelte-forms";
  import { notifications } from "@whizzes/svelte-notifications";
  import { onMount } from "svelte";
  import * as Yup from "yup";

  import { createHeader } from "$lib/utils/basic-auth";
  import { LoginError, type ErrorMessages } from "./shared";
  import { UserErrorCode } from "$lib/graphql/schema";
  import TextField from "$lib/components/TextField.svelte";
  import Button from "$lib/components/Button.svelte";

  import type { Unsplash } from "../../api/unsplash/+server";

  let cover: Unsplash | null = null;
  let error: string | null = null;

  onMount(async () => {
    const response = await fetch("/api/unsplash");
    const data = await response.json();

    cover = data.data;
  });

  const errorMessages: ErrorMessages = {
    [LoginError.MissingCredentials]: "Please enter your email and password",
    [UserErrorCode.Unauthorized]: "Invalid email or password",
  };

  const { handleSubmit, values, errors, isSubmitting } = newForm({
    initialValues: {
      email: "",
      password: "",
    },
    validationSchema: Yup.object({
      email: Yup.string().email().required("Email is required"),
      password: Yup.string().required("Password is required"),
    }),
    onSubmit: async ({ email, password }) => {
      const basicAuth = createHeader(email, password);
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

<div class="flex flex-col justify-center h-full">
  <div class="py-4">
    <h1 class="font-semibold">Welcome Back!</h1>
    <span>
      Log in to your account to continue
    </span>
  </div>
  <form class="flex flex-col py-4 space-y-4" on:submit={handleSubmit}>
    <TextField
      type="text"
      name="userId"
      error={$errors.email}
      bind:value={$values.email}
      placeholder="Email"
    />
    <TextField
      type="password"
      name="password"
      error={$errors.password}
      bind:value={$values.password}
      placeholder="Password"
    />
    <div>
      <label for="remeber_credentials">
        <input type="checkbox" name="remeber_credentials" />
        <small>Keep me logged in</small>
      </label>
    </div>
    <p class="error_message" hidden={!error}>{error}</p>
    <Button type="submit" variant="primary">
      Log in
    </Button>
  </form>
  <small class="text-gray-600">
    Don't have an account? <a class="text-blue-600 underline" href="/signup">Sign up</a>
  </small>
</div>
