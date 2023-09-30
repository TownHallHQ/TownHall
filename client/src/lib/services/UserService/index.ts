import {
  UserRegisterDocument
} from '$lib/graphql/schema';

import type { Client } from '@urql/core';
import type {
  AccessToken,
  CurrentUserFragment,
  UserErrorCode,
  UserRegisterInput,
} from '$lib/graphql/schema';
import { GraphQLError } from '$lib/utils/graphql';

export class UserError extends GraphQLError<UserErrorCode> {}

export class UserService {
  static async userRegister(
    urqlClient: Client,
    input: UserRegisterInput
  ): Promise<CurrentUserFragment> {
    const response = await urqlClient
      .mutation(
        UserRegisterDocument,
        {
          input,
        },
        {
          requestPolicy: 'network-only', // We dont want to cache this request
        },
      )
      .toPromise();

    if (response?.error || response?.data?.userRegister?.error) {
      if (response?.data?.userRegister?.error) {
        const error: UserError = response.data.userRegister.error;

        throw UserError.new(error.code, error.message);
      }

      throw response?.error;
    }

    return response.data.userRegister.user;
  }
}
