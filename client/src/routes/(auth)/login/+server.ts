import { createClient, cacheExchange, fetchExchange } from '@urql/core';

import { parseHeader } from '$lib/utils/basic-auth';

import type { Cookies } from '@sveltejs/kit';
import { JsonResponse, StatusCode } from '$lib/utils/http';
import { AuthError, AuthService } from '$lib/services/AuthService';

const AUTH_TOKEN_COOKIE_MAX_AGE_DAYS = 30;

export enum LoginError {
  MissingCredentials = 'MISSING_CREDENTIALS',
  InvalidCredentials = 'INVALID_CREDENTIALS',
  Unknown = 'UNKNOWN',
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
      return JsonResponse.error<LoginError>(
        StatusCode.BadRequest,
        LoginError.MissingCredentials,
      );
    }

    const urqlClient = createClient({
      url: import.meta.env.VITE_GRAPHQL_URL,
      exchanges: [cacheExchange, fetchExchange],
    });
    const tokens = await AuthService.tokenCreate(
      urqlClient,
      username,
      password,
    );

    cookies.set('accessToken', tokens.accessToken, {
      path: '/',
      httpOnly: true,
      sameSite: 'strict',
      secure: process.env.NODE_ENV === 'production',
      maxAge: 60 * 60 * 24 * AUTH_TOKEN_COOKIE_MAX_AGE_DAYS,
    });

    return new Response(null, {
      status: 201,
    });
  } catch (err) {
    if (err instanceof AuthError) {
      return JsonResponse.error<LoginError>(
        StatusCode.Unauthorized,
        LoginError.InvalidCredentials,
      );
    }

    return JsonResponse.error<LoginError>(
      StatusCode.InternalServerError,
      LoginError.Unknown,
    );
  }
};
