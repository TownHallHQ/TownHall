import { redirect } from '@sveltejs/kit';

import type { CurrentUserFragment } from '$lib/graphql/schema';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = ({
  locals,
}: {
  locals: {
    accessToken?: string;
    user?: CurrentUserFragment;
  };
}) => {
  if (locals.user || locals.accessToken) {
    throw redirect(302, '/');
  }

  return {
    accessToken: locals.accessToken,
    user: locals.user,
  };
};
