import { redirect } from '@sveltejs/kit';

import type { RequestHandler } from './$types';

export const GET = (({ cookies }) => {
  cookies.delete('accessToken');

  throw redirect(302, '/login');
}) satisfies RequestHandler;
