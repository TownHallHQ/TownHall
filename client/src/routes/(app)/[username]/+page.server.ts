import { cacheExchange, createClient, fetchExchange } from '@urql/core';
import { error } from '@sveltejs/kit';

import type { PageServerLoad } from './$types';
import type { Client } from '@urql/core';
import {
  GetUsersDocument,
  type GetUsersQueryVariables,
  type User,
} from '$lib/graphql/schema';

async function getUser(urqlClient: Client, username: string): Promise<User> {
  const variables: GetUsersQueryVariables = {
    filter: { username: username },
  };

  const response = await urqlClient
    .query(
      GetUsersDocument,
      variables,

      {
        requestPolicy: 'network-only',
      },
    )
    .toPromise();

  if (response?.error || response?.data?.me?.error) {
    throw new Error('Failed');
  }

  return response?.data?.user?.nodes?.at(0);
}

export const load: PageServerLoad = async ({ params }) => {
  const urqlClient = createClient({
    url: import.meta.env.VITE_GRAPHQL_URL,
    exchanges: [cacheExchange, fetchExchange],
  });

  const user = await getUser(urqlClient, params.username);

  if (user) {
    return {
      profileUser: user,
    };
  }
  throw error(404, 'User not found');
};
