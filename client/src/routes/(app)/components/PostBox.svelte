<script lang="ts">
  import * as Yup from 'yup';
  import { newForm } from '@whizzes/svelte-forms';

  import { page } from '$app/stores';
  import Button from '$lib/components/Button.svelte';
  import Card from '$lib/components/Card.svelte';
  import TextField from '$lib/components/TextField.svelte';
  import { notifications } from '@whizzes/svelte-notifications';
  import Avatar from '$lib/components/Avatar.svelte';

  let customClassName: string | null = null;
  export { customClassName as class };

  const { handleSubmit, values, errors, isSubmitting } = newForm({
    initialValues: {
      title: '',
      content: '',
    },
    validationSchema: Yup.object({
      title: Yup.string().required('content is required'),
      content: Yup.string().required('content is required'),
    }),
    onSubmit: async (values, helpers) => {
      const response = await fetch('/create-post', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(values),
      });

      if (response.ok) {
        const data = await response.json();

        console.log(data);

        notifications.notifySuccess('Post create successfully');
        helpers.setFieldValue('title', '');
        helpers.setFieldError('content', '');
      } else {
        notifications.notifyFailure('A error occurred');
      }
    },
  });
</script>

<Card class={customClassName}>
  <form on:submit={handleSubmit}>
    <div class="flex space-x-4">
      <Avatar size="sm" />
      <div class="w-full">
        <TextField
          class="w-full"
          name="title"
          type="text"
          placeholder="My post"
          error={$errors.title}
          bind:value={$values.title}
        />
      </div>
    </div>
    <TextField
      class="w-[calc(100%-44px)] mt-3"
      name="content"
      type="text"
      placeholder="Whats on your mind?"
      error={$errors.content}
      bind:value={$values.content}
    />
    <div class="flex mt-2 justify-end">
      <Button type="submit" isLoading={$isSubmitting} variant="primary"
        >Publish</Button
      >
    </div>
  </form>
</Card>
