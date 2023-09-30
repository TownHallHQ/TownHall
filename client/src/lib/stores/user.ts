import { writable } from "svelte/store";

import type { CurrentUserFragment } from "$lib/graphql/schema";
import type { Unsubscriber, Writable } from "svelte/store";
import type { Readable, Subscriber } from "svelte/motion";
import type { Client } from "@urql/core";
import { UserService } from "$lib/services/UserService";

export interface UserStoreMethods {
  /**
   * Sets the initial state for the `UserStore`
   */
  init(user: CurrentUserFragment): void;

  /**
   *
   * @param urqlClient 
   * @param file 
   *
   * Updates the avatar of this user
   */
  updateAvatar(urqlClient: Client, file: File): Promise<void>;
}

export class UserStore implements UserStoreMethods {
  private inner: Writable<CurrentUserFragment | null>;

  constructor() {
    this.inner = writable(null);
  }

  public subscribe(run: Subscriber<CurrentUserFragment | null>): Unsubscriber {
    return this.inner.subscribe(run);
  }

  public init(user: CurrentUserFragment): void {
    this.inner.set(user);
  }

  public async updateAvatar(urqlClient: Client, file: File): Promise<void> {
    const user = await UserService.userAvatarUpdate(urqlClient, file);
    this.inner.set(user);
  }
}

export const userStore = new UserStore() as unknown as Readable<CurrentUserFragment | null> & UserStoreMethods;
