import { createApi } from 'unsplash-js';

import { UNSPLASH_ACCESS_KEY } from '$env/static/private';
import { JsonResponse, StatusCode } from '$lib/utils/http';

import type { RequestEvent } from './$types';
import { UnsplashErrorCode } from '$lib/services/Unsplash';

export type UnsplashAuthor = {
  id: string;
  name: string;
  username: string;
  avatar: string;
}

export type UnsplashImage = {
  url: URL;
  author: UnsplashAuthor;
};

const UNSPLASH_DAILY_BACKGROUND_COOKIE = 'UNSPLASH_DAILY_BACKGROUND';

export async function GET(event: RequestEvent) {
  const UNSPLASH_DAILY_BACKGROUND = event.cookies.get(UNSPLASH_DAILY_BACKGROUND_COOKIE);

  if (UNSPLASH_DAILY_BACKGROUND) {
    const body = JSON.parse(UNSPLASH_DAILY_BACKGROUND);

    return JsonResponse.success<UnsplashImage>(body);
  }

  if (!UNSPLASH_ACCESS_KEY) {
    return JsonResponse.error<UnsplashErrorCode>(StatusCode.InternalServerError, UnsplashErrorCode.MissingApiToken, 'UNSPLASH_ACCESS_KEY environment variable is not set');
  }

  const unsplash = createApi({
    accessKey: UNSPLASH_ACCESS_KEY,
  });

  const result = await unsplash.photos.getRandom({
    query: 'night lights',
  });

  if (result.status === 200) {
    const image = Array.isArray(result?.response)
      ? result?.response[0]
      : result?.response;

    if (image) {
      const today = new Date();

      today.setDate(today.getDate() + 1);

      const body: UnsplashImage = {
        url: new URL(image.urls.regular),
        author: {
          id: image.user.id,
          username: image.user.username,
          name: image.user.name,
          avatar: image.user.profile_image?.medium,
        },
      };

      event.cookies.set('UNSPLASH_DAILY_BACKGROUND', JSON.stringify(body), {
        expires: today,
        path: '/',
        httpOnly: true,
        sameSite: 'strict',
        secure: process.env.NODE_ENV === 'production',
      });

      return JsonResponse.success<UnsplashImage>(body);
    }
  }

  return JsonResponse.error<UnsplashErrorCode>(StatusCode.InternalServerError, UnsplashErrorCode.Unknown, 'Failed to fetch Unsplash background');
}
