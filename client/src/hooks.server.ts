import { createClient, cacheExchange, fetchExchange } from '@urql/core';

import type { Handle } from '@sveltejs/kit';
import { AuthService } from '$lib/services/AuthService';

export const handle: Handle = async ({ event, resolve }) => {
  try {
    const accessToken = event.cookies.get('accessToken');

    if (!accessToken) {
      return await resolve(event);
    }

    if (event.locals.accessToken && event.locals.user) {
      return await resolve(event);
    }

    const urqlClient = createClient({
      url: import.meta.env.VITE_GRAPHQL_URL,
      exchanges: [cacheExchange, fetchExchange],
    });

    const userDetials = await AuthService.whoami(urqlClient, accessToken);

    if (userDetials && accessToken) {
      event.locals.accessToken = accessToken;
      event.locals.user = userDetials;
    }

    return await resolve(event);
  } catch (err) {
    event.locals.user = null;
    event.locals.accessToken = null;

    return await resolve(event);
  }
};
