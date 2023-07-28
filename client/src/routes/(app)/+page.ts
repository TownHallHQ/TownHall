import type { PostEdge } from '$lib/graphql/schema.js';

/** @type {import('./$types').PageLoad} */
export async function load({ fetch }) {
  const response = await fetch('/get-posts');
  const data = await response.json();
  return {
    posts: data as PostEdge[],
  };
}
