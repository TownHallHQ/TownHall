import { UserCreateDocument } from '$lib/graphql/schema';
import { cacheExchange, createClient, fetchExchange } from '@urql/core';

import type { Cookies } from '@sveltejs/kit';
import type { Client } from '@urql/core';
import { createToken } from '$auth/login/+server';

export type CreateUserPayload = {
  name: string;
  surname: string;
  username: string;
  email: string;
  password: string;
};

async function createUser(
  urqlClient: Client,
  payload: CreateUserPayload
): Promise<{ id: string }> {
  const response = await urqlClient
    .mutation(
      UserCreateDocument,
      { input: payload },
      {
        requestPolicy: 'network-only',
      }
    )
    .toPromise();

  if (response?.error || response?.data?.tokenCreate?.error) {
    if (response?.data?.userCreate?.error) {
      const error = response?.data?.userCreate?.error;

      throw new Error(error);
    }

    throw response?.error;
  }

  return response?.data?.userCreate?.user;
}

export const POST = async ({
  cookies,
  request,
}: {
  cookies: Cookies;
  request: Request;
}) => {
  try {
    const requestBody: CreateUserPayload = await request.json();
    const urqlClient = createClient({
      url: import.meta.env.VITE_GRAPHQL_URL,
      exchanges: [cacheExchange, fetchExchange],
    });
    await createUser(urqlClient, requestBody);

    const tokens = await createToken(
      urqlClient,
      requestBody.email,
      requestBody.password
    );

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

    return new Response(
      JSON.stringify({
        message: 'Invalid Credentials',
      }),
      {
        status: 403,
      }
    );
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
