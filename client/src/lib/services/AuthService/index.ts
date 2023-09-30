import {
  MeDocument,
  TokenCreateDocument
} from '$lib/graphql/schema';

import type { Client } from '@urql/core';
import type {
  AccessToken,
  AuthErrorCode,
  CurrentUserFragment,
} from '$lib/graphql/schema';
import { GraphQLError } from '$lib/utils/graphql';

export class AuthError extends GraphQLError<AuthErrorCode> {}

export class AuthService {
  static async tokenCreate(
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
          requestPolicy: 'network-only', // We dont want to cache this request
        },
      )
      .toPromise();

    if (response?.error || response?.data?.tokenCreate?.error) {
      if (response?.data?.tokenCreate?.error) {
        const error: AuthError = response.data.tokenCreate.error;

        throw AuthError.new(error.code, error.message);
      }

      throw response?.error;
    }

    return response.data.tokenCreate.token;
  }

  static async whoami(
    urqlClient: Client,
    accessToken: string,
  ): Promise<CurrentUserFragment> {
    const response = await urqlClient
      .query(
        MeDocument,
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
}
