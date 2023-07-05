// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces

import type { CurrentUserFragmentFragment } from '$lib/graphql/schema';

declare global {
  namespace App {
    // interface Error {}
    // interface Platform {}

    interface PageData {
      accessToken?: string | null;
      user?: CurrentUserFragmentFragment | null;
    }

    interface Locals {
      accessToken?: string | null;
      user?: CurrentUserFragmentFragment | null;
    }
  }
}
