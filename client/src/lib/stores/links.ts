import { notifications } from '@whizzes/svelte-notifications';
import { get, writable } from 'svelte/store';

import type { CreatedLinkFragment } from '$lib/graphql/schema';

import {
  LinkCreateDocument,
  GetCurrentUserDocument,
} from '$lib/graphql/schema';

import type { Client } from '@urql/svelte';
import type { Subscriber, Readable, Writable } from 'svelte/store';

export type CreateLinkPayload = {
  title: string;
  url: string;
};

export type LinkStoreMethods = {
  createLink: (urqlClient: Client, payload: CreateLinkPayload) => Promise<void>;
  load: (urqlClient: Client) => Promise<void>;
};

export class LinkStore {
  private _links: Writable<CreatedLinkFragment[]>;

  constructor() {
    this._links = writable<CreatedLinkFragment[]>([]);
  }

  get links(): CreatedLinkFragment[] {
    return get(this._links);
  }

  public suscribe(run: Subscriber<CreatedLinkFragment[]>) {
    return this._links.subscribe(run);
  }

  public async load(urqlClient: Client): Promise<void> {
    const response = await urqlClient
      .query(GetCurrentUserDocument, {})
      .toPromise();

    if (response.error) {
      console.error(response.error);
      throw new Error('Failed to load links');
    }

    const links = response.data?.currentUser?.links ?? [];
    this._links.set(links);

    return links;
  }

  public async createLink(
    urqlClient: Client,
    payload: CreateLinkPayload,
  ): Promise<void> {
    const result = await urqlClient
      .mutation(LinkCreateDocument, {
        input: {
          title: payload.title,
          url: payload.url,
        },
      })
      .toPromise();

    if (result.error || result.data?.linkCreate?.error) {
      return notifications.notifyFailure(
        result.data?.linkCreate?.error?.message,
      );
    }

    notifications.notifySuccess(
      `The url has been created with the hash: ${result.data?.linkCreate?.link?.ulid}`,
    );
  }
}

export const linkStore = new LinkStore() as unknown as Readable<
  CreatedLinkFragment[]
> &
  LinkStoreMethods;
