import { GetPostsDocument, type PostConnection } from '$lib/graphql/schema';
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

  return (response?.data?.posts as PostConnection).nodes;
}

export const GET = async ({ request }: { request: Request }) => {
  try {
    const url = new URL(request.url);
    const pageNumber = Number(url.searchParams.get('first')) || undefined;
    const afterPostPxid = url.searchParams.get('after') || undefined;

    const urqlClient = createClient({
      url: import.meta.env.VITE_GRAPHQL_URL,
      exchanges: [cacheExchange, fetchExchange],
    });

    const posts = await getPosts(urqlClient, {
      first: pageNumber,
      after: afterPostPxid,
    });

    return new Response(JSON.stringify(posts), { status: 200 });
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
