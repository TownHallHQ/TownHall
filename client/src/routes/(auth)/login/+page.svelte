<script lang="ts">
  import { newForm } from "@whizzes/svelte-forms";
  import { notifications } from "@whizzes/svelte-notifications";
  import * as Yup from "yup";

  import { createHeader } from "$lib/utils/basic-auth";
  import TextField from "$lib/components/TextField.svelte";
  import Button from "$lib/components/Button.svelte";
  import { LoginError } from "./+server";

  import type { HttpErrorResponse } from "$lib/utils/http";

  let error: string | null = null;

  const { handleSubmit, values, errors } = newForm({
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
        return;
      }

      const data: HttpErrorResponse<LoginError> = await response.json();

      switch (data.code) {
        case LoginError.MissingCredentials:
          error = "Please enter your email and password";
        case LoginError.InvalidCredentials:
          error = "Invalid email or password";
        case LoginError.Unknown:
          error = "An unknown error occurred";
      }
    },
  });
</script>

<div class="flex flex-col justify-center h-full">
  <div class="text-lg py-4">
    <h1 class="font-semibold">Welcome Back!</h1>
    <span> Log in to your account to continue </span>
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
    <Button type="submit" variant="primary">Log in</Button>
  </form>
  <small class="text-gray-600">
    Don't have an account? <a class="text-blue-600 underline" href="/signup"
      >Sign up</a
    >
  </small>
</div>
