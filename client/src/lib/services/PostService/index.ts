import { PostCreateDocument } from '$lib/graphql/schema';

import type { Client } from '@urql/core';
import type {
  PostCreateFieldsFragment,
  PostCreateInput,
  PostErrorCode,
} from '$lib/graphql/schema';

export class PostError extends Error {
  constructor(code: PostErrorCode, message: string) {
    super(`PostError: ${code} - ${message}`);
    this.name = 'PostError';
  }
}

export class PostService {
  static async create(
    urqlClient: Client,
    input: PostCreateInput,
  ): Promise<PostCreateFieldsFragment> {
    const response = await urqlClient
      .mutation(PostCreateDocument, {
        input,
      })
      .toPromise();

    if (response.error) {
      throw new Error(response.error.message);
    }

    if (response.data?.postCreate?.post) {
      return response.data?.postCreate?.post;
    }

    const { code, message } = response.data?.postCreate?.error || {
      code: 'UNKNOWN',
      message: 'Unknown error',
    };

    throw new PostError(code, message);
  }
}
