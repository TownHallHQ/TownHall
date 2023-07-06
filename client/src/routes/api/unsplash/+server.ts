import { createApi } from 'unsplash-js';
import type { RequestEvent } from './$types';
import { UNSPLASH_ACCESS_KEY } from '$env/static/private';

export type Unsplash = {
  url: string;
  author: {
    id: string;
    name: string;
    username: string;
    url: string;
    avatar: string;
  };
};

export async function GET(event: RequestEvent) {
  const UNSPLASH_DAILY_BACKGROUND = event.cookies.get(
    'UNSPLASH_DAILY_BACKGROUND'
  );

  if (UNSPLASH_DAILY_BACKGROUND) {
    return new Response(
      JSON.stringify({
        statusCode: 200,
        data: JSON.parse(UNSPLASH_DAILY_BACKGROUND),
        error: null,
        success: true,
      })
    );
  }

  const unsplash = createApi({
    accessKey: UNSPLASH_ACCESS_KEY || '',
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

      const response = {
        url: image.urls?.regular,
        author: {
          id: image.user?.id,
          username: image.user?.username,
          name: image.user?.name,
          url: image.user?.portfolio_url,
          avatar: image.user?.profile_image?.medium,
        },
      };

      event.cookies.set('UNSPLASH_DAILY_BACKGROUND', JSON.stringify(response), {
        expires: today,
        path: '/',
        httpOnly: true,
        sameSite: 'strict',
        secure: process.env.NODE_ENV === 'production',
      });

      return new Response(
        JSON.stringify({
          statusCode: 200,
          data: response,
          success: true,
          error: null,
        })
      );
    }
  }

  return new Response(
    JSON.stringify({
      statusCode: result.status,
      error: {
        message: result.errors?.[0] || 'An error ocurred fetching the resource',
      },
      data: null,
      success: false,
    })
  );
}
