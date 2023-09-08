<script lang="ts">
  import * as Yup from 'yup';
  import { notifications } from '@whizzes/svelte-notifications';
  import { newForm } from '@whizzes/svelte-forms';
  import { getContextClient } from '@urql/svelte';

  import Button from '$lib/components/Button.svelte';
  import Card from '$lib/components/Card.svelte';
  import TextField from '$lib/components/TextField.svelte';
  import Avatar from '$lib/components/Avatar.svelte';
  import { PostError, PostService } from '$lib/services/PostService';
    import { createEventDispatcher } from 'svelte';

  let className: string = '';
  export { className as class };

  const dispatch = createEventDispatcher();
  const urqlClient = getContextClient();
  const { handleSubmit, values, errors, isSubmitting } = newForm({
    initialValues: {
      title: '',
    },
    validationSchema: Yup.object({
      title: Yup.string().required('content is required'),
    }),
    onSubmit: async (values) => {
      try {
        const post = await PostService.create(urqlClient, {
          title: values.title,
        });

        dispatch('postCreated', {
          post
        });

        notifications.notifySuccess('Post published with success');
      } catch (error) {
        if (error instanceof PostError) {
          notifications.notifyFailure(error.message);
          return;
        }

        notifications.notifyFailure('Something went wrong');
      }
    },
  });
</script>

<div class="{className}">
  <Card class="h-min">
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
      <div class="flex mt-2 justify-end">
        <Button type="submit" isLoading={$isSubmitting} variant="primary"
          >Publish</Button
        >
      </div>
    </form>
  </Card>
</div>
