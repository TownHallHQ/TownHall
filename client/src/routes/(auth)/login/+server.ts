import { TokenCreateDocument } from '$lib/graphql/schema';
import { createClient, cacheExchange, fetchExchange } from '@urql/core';

import { parseHeader } from '$lib/utils/basic-auth';

import type { Cookies } from '@sveltejs/kit';
import type { Client } from '@urql/core';
import type { AccessToken, UserError } from '$lib/graphql/schema';
import { LoginError } from './shared';

export async function createToken(
  urqlClient: Client,
  email: string,
  password: string,
): Promise<AccessToken> {
  const response = await urqlClient
    .mutation(
      TokenCreateDocument,
      {
        email,
        password,
      },
      {
        requestPolicy: 'network-only',
      },
    )
    .toPromise();

  if (response?.error || response?.data?.tokenCreate?.error) {
    if (response?.data?.tokenCreate?.error) {
      const error: UserError = response.data.tokenCreate.error;

      throw error;
    }

    throw response?.error;
  }

  return response?.data?.tokenCreate?.token;
}

export const POST = async ({
  cookies,
  request,
}: {
  cookies: Cookies;
  request: Request;
}) => {
  try {
    const { username, password } = parseHeader(request);

    if (!username || !password) {
      return new Response(
        JSON.stringify({
          error: LoginError.MissingCredentials,
          message: 'Missing username and/or password credentials',
        }),
        {
          status: 422,
        },
      );
    }

    const urqlClient = createClient({
      url: import.meta.env.VITE_GRAPHQL_URL,
      exchanges: [cacheExchange, fetchExchange],
    });
    const tokens = await createToken(urqlClient, username, password);

    if (tokens.accessToken) {
      cookies.set('accessToken', tokens.accessToken, {
        path: '/',
        httpOnly: true,
        sameSite: 'strict',
        secure: process.env.NODE_ENV === 'production',
        // Expires in a month
        maxAge: 60 * 60 * 24 * 30,
      });

      return new Response(null, {
        status: 201,
      });
    }
  } catch (err) {
    console.error(err);

    return new Response(
      JSON.stringify({
        error: (err as { message: string; code: string })?.code,
        message: (err as { message: string; code: string })?.message,
      }),
      {
        status: 500,
      },
    );
  }
};
