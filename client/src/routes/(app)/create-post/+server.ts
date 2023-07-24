import { PostCreateDocument } from '$lib/graphql/schema';

import {
  createClient,
  type Client,
  cacheExchange,
  fetchExchange,
} from '@urql/core';

import type { Post } from '$lib/graphql/schema';
import type { Cookies } from '@sveltejs/kit';

export type CreatePostPayload = {
  title: string;
  content: string;
  parent_id: string;
};

async function createPost(
  urqlClient: Client,
  accessToken: string,
  payload: CreatePostPayload
): Promise<Post> {
  const response = await urqlClient
    .mutation(
      PostCreateDocument,
      { input: payload },
      {
        requestPolicy: 'network-only',
        fetchOptions: {
          headers: {
            Authorization: `JWT ${accessToken}`,
          },
        },
      }
    )
    .toPromise();

  if (response?.error || response?.data?.postCreate?.error) {
    if (response?.data?.postCreate?.error) {
      const error = response?.data?.postCreate?.error;

      throw new Error(error);
    }

    throw response?.error;
  }

  return response?.data?.postCreate?.post;
}

export const POST = async ({
  cookies,
  request,
}: {
  cookies: Cookies;
  request: Request;
}) => {
  try {
    const requestBody: CreatePostPayload = await request.json();
    const accessToken = cookies.get('accessToken') || '';
    const urqlClient = createClient({
      url: import.meta.env.VITE_GRAPHQL_URL,
      exchanges: [cacheExchange, fetchExchange],
    });

    const post = await createPost(urqlClient, accessToken, requestBody);

    return new Response(JSON.stringify(post), { status: 201 });
  } catch (err) {
    console.error(err);

    return new Response(
      JSON.stringify({
        message: 'Internal Server Error',
        error: (err as { message: string })?.message,
      }),
      {
        status: 500,
      }
    );
  }
};
