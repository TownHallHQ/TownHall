import { UserUpdateDocument } from '$lib/graphql/schema';

import type { Client } from '@urql/core';
import type {
  UserUpdateInput,
  UserFragment,
  UserErrorCode,
} from '$lib/graphql/schema';

export class UserError extends Error {
  constructor(code: UserErrorCode, message: string) {
    super(`UserError: ${code} - ${message}`);
    this.name = 'UserError';
  }
}

export class UserService {
  static async update(
    urqlClient: Client,
    id: string,
    input: UserUpdateInput
  ): Promise<UserFragment> {
    const response = await urqlClient
      .mutation(UserUpdateDocument, {
        id,
        input,
      })
      .toPromise();

    if (response.error) {
      throw new Error(response.error.message);
    }

    if (response.data?.userUpdate?.user) {
      return response.data?.userUpdate?.user;
    }

    const { code, message } = response.data?.userUpdate?.error || {
      code: 'UNKNOWN',
      message: 'Unknown error',
    };

    throw new UserError(code, message);
  }
}
