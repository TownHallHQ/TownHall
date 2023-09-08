import {
  GetPostsDocument,
  GetUserPostsDocument,
  type GetUserPostsQueryVariables,
  type PostConnection,
} from '$lib/graphql/schema';
import { Client, cacheExchange, createClient, fetchExchange } from '@urql/core';

async function getPosts(
  urqlClient: Client,
  variables: {
    first?: number;
    last?: number;
    after?: string | null;
    before?: string | null;
  },
) {
  const response = await urqlClient
    .query(GetPostsDocument, variables, {
      requestPolicy: 'network-only',
    })
    .toPromise();

  if (response?.error || response?.data?.posts?.error) {
    if (response?.data?.posts?.error) {
      const error = response?.data?.posts?.error;

      throw new Error(error);
    }

    throw response?.error;
  }

  return response?.data?.posts as PostConnection;
}

async function getUserPosts(
  urqlClient: Client,
  variables: GetUserPostsQueryVariables,
) {
  const response = await urqlClient
    .query(GetUserPostsDocument, variables, {
      requestPolicy: 'network-only',
    })
    .toPromise();

  if (response?.error || response?.data?.posts?.error) {
    if (response?.data?.posts?.error) {
      const error = response?.data?.posts?.error;

      throw new Error(error);
    }

    throw response?.error;
  }

  return response?.data?.user?.nodes.at(0).posts as PostConnection;
}

export const GET = async ({ request }: { request: Request }) => {
  try {
    const url = new URL(request.url);
    const pageNumber = Number(url.searchParams.get('first')) || undefined;
    const afterPostPxid = url.searchParams.get('after') || undefined;
    const username = url.searchParams.get('username') || undefined;

    const urqlClient = createClient({
      url: import.meta.env.VITE_GRAPHQL_URL,
      exchanges: [cacheExchange, fetchExchange],
    });

    if (username) {
      const userPosts = await getUserPosts(urqlClient, {
        username: username,
        first: pageNumber,
        after: afterPostPxid,
      });

      return new Response(JSON.stringify(userPosts), { status: 200 });
    } else {
      const posts = await getPosts(urqlClient, {
        first: pageNumber,
        after: afterPostPxid,
      });

      return new Response(JSON.stringify(posts), { status: 200 });
    }
  } catch (err) {
    console.log(err);

    return new Response(
      JSON.stringify({
        message: 'Internal Server Error',
        error: (err as { message: string })?.message,
      }),
      { status: 500 },
    );
  }
};
