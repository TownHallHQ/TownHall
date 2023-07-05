import { createClient, cacheExchange, fetchExchange } from '@urql/core';
import { GetCurrentUserDocument } from '$lib/graphql/schema';

import type { Handle } from '@sveltejs/kit';
import type { Client } from '@urql/core';
import type { User } from '$lib/graphql/schema';

async function getUserDetails(
  urqlClient: Client,
  accessToken: string,
): Promise<User> {
  const response = await urqlClient
    .query(
      GetCurrentUserDocument,
      {},
      {
        requestPolicy: 'network-only',
        fetchOptions: {
          headers: {
            Authorization: `JWT ${accessToken}`,
          },
        },
      },
    )
    .toPromise();

  if (response?.error || response?.data?.me?.error) {
    throw new Error('Failed');
  }

  return response?.data?.me?.user;
}

export const handle: Handle = async ({ event, resolve }) => {
  try {
    const accessToken = event.cookies.get('accessToken');

    if (!accessToken) {
      return await resolve(event);
    }

    const urqlClient = createClient({
      url: import.meta.env.VITE_GRAPHQL_URL,
      exchanges: [cacheExchange, fetchExchange],
    });

    const userDetials = await getUserDetails(urqlClient, accessToken);

    if (userDetials && accessToken) {
      (event.locals as App.PageData).accessToken = accessToken;
      (event.locals as App.PageData).user = userDetials;
    }

    return await resolve(event);
  } catch (err) {
    console.log(err);
    event.locals.user = null;
    event.locals.accessToken = null;

    return await resolve(event);
  }
};
