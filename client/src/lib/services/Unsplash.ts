import type { UnsplashImage } from '$routes/api/unsplash/+server';

export enum UnsplashErrorCode {
  MissingApiToken = 0,
  Unknown = 1,
}

export class UnsplashService {
  /**
   * Fetches a random background image from Unsplash.
   *
   * This method relies on the `UNSPLASH_ACCESS_KEY` environment variable
   * to be set.
   */
  static async fetchUnsplashBackground(): Promise<UnsplashImage | null> {
    const response = await fetch('/api/unsplash', {
      method: 'GET',
    });

    if (response.ok) {
      return response.json();
    }

    return null;
  }
}
